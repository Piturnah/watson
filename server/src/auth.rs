use std::{collections::HashMap, sync::Arc};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{Json, Response},
};
use chrono::{offset::Utc, DateTime, Days};
use diesel::prelude::*;
use rand::rngs::OsRng;
use serde::Deserialize;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{establish_connection, internal_error, models::AccessToken};

#[derive(Clone)]
pub struct Session {
    user_id: Uuid,
    expires: DateTime<Utc>,
}

pub type Sessions = Arc<RwLock<HashMap<Uuid, Session>>>;

/// Middleware for session authentication.
pub async fn auth(
    State(sessions): State<Sessions>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let session_id = request
        .headers()
        .get("Authorization")
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "No session active".to_string()))?
        .to_str()
        .map_err(internal_error)?;

    let sessions_rlock = sessions.read().await;
    let session = sessions_rlock
        .get(&Uuid::parse_str(session_id).map_err(internal_error)?)
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "No session active".to_string()))?;

    if Utc::now()
        .signed_duration_since(session.expires)
        .num_microseconds()
        .unwrap_or(1)
        > 0
    {
        return Err((StatusCode::UNAUTHORIZED, "Session expired".to_string()));
    }

    // Ok, we are authorized!
    request
        .headers_mut()
        .insert("user_id", session.user_id.to_string().parse().unwrap());

    Ok(next.run(request).await)
}

#[derive(Deserialize)]
pub struct RegisterRequestBody {
    req_token: Uuid,
    req_email: String,
    req_password: String,
}

pub async fn register(
    Json(RegisterRequestBody {
        req_token,
        req_email,
        req_password,
    }): Json<RegisterRequestBody>,
) -> Result<(), (StatusCode, String)> {
    use crate::schema::{access_tokens, users};
    let mut conn = establish_connection();
    let token = access_tokens::table::find(access_tokens::table, req_token)
        .select(AccessToken::as_select())
        .first(&mut conn)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token.".to_string()))?;
    if token.redeemed {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Token already redeemed.".to_string(),
        ));
    }

    // Wooo! New user!
    diesel::update(access_tokens::table.find(req_token))
        .set(access_tokens::redeemed.eq(true))
        .execute(&mut conn)
        .map_err(internal_error)?;

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(req_password.as_bytes(), &salt)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't hash password".to_string(),
            )
        })?
        .to_string();

    diesel::insert_into(users::table)
        .values((
            users::name.eq(&token.name),
            users::email.eq(&req_email),
            users::password.eq(&hashed_password),
        ))
        .execute(&mut conn)
        .map_err(internal_error)?;
    Ok(())
}

#[derive(Deserialize)]
pub struct AuthRequestBody {
    req_email: String,
    req_password: String,
}

pub async fn login(
    State(sessions): State<Sessions>,
    Json(AuthRequestBody {
        req_email,
        req_password,
    }): Json<AuthRequestBody>,
) -> Result<String, (StatusCode, String)> {
    use crate::schema::users::*;
    let mut conn = establish_connection();

    let (user_id, password_hash): (Uuid, Option<String>) =
        table::filter(table, email.eq(req_email))
            .select((id, password.nullable()))
            .first(&mut conn)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

    // TODO: Deal with null password case.
    let Some(password_hash) = password_hash else {
        return Err((
            StatusCode::UNAUTHORIZED,
            "This user has no password set! Contact administrator.".to_string(),
        ));
    };

    let parsed_hash = PasswordHash::new(&password_hash).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal error occurred".to_string(),
        )
    })?;

    if Argon2::default()
        .verify_password(req_password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err((StatusCode::UNAUTHORIZED, "Incorrect password.".to_string()));
    }

    // XXX: ONLY FROM THIS POINT ON ARE WE AUTHORIZED.
    let mut sessions_wlock = sessions.write().await;
    let session_id = Uuid::new_v4();
    let expires = Utc::now()
        .checked_add_days(Days::new(1))
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "Funky time".to_string()))?;
    sessions_wlock.insert(session_id, Session { user_id, expires });

    // TODO: I tried to do this with Set-Cookie header. But I am too stupid to work it out.
    Ok(session_id.to_string())
}
