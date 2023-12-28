use std::{collections::HashMap, env, sync::Arc};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{Json, Response},
};
use base64::Engine;
use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use jsonwebtoken::{jwk::Jwk, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{establish_connection, internal_error};

pub type Sessions = Arc<RwLock<HashMap<String, DateTime<Utc>>>>;

/// Middleware for token authentication.
pub async fn auth(
    State(sessions): State<Sessions>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let bearer_token = request
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.split("Bearer ").nth(1))
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "no token found in headers".to_string(),
            )
        })?;

    let mut sessions_wlock = sessions.write().await;
    let user_sub = if let Some(datetime) = sessions_wlock.get_mut(bearer_token) {
        // Check for session expiry.
        if Utc::now()
            .signed_duration_since(datetime.clone())
            .abs()
            .num_hours()
            > 1
        {
            return Err((StatusCode::UNAUTHORIZED, "session expired".to_string()));
        }

        *datetime = Utc::now();
        let payload = bearer_token
            .split('.')
            .nth(1)
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "invalid token".to_string()))?;
        let user: UserInfo = serde_json::from_str(
            &String::from_utf8(
                base64::engine::general_purpose::STANDARD_NO_PAD
                    .decode(payload)
                    .map_err(internal_error)?,
            )
            .map_err(internal_error)?,
        )
        .map_err(internal_error)?;
        user.sub
    } else {
        let user = validate_token(GoogleCredential {
            credential: bearer_token.to_string(),
        })
        .await?;
        sessions_wlock.insert(bearer_token.to_string(), Utc::now());
        user.sub
    };

    request
        .headers_mut()
        .insert("user_sub", user_sub.parse().unwrap());
    Ok(next.run(request).await)
}

#[derive(Deserialize, Debug)]
pub struct GoogleCredential {
    credential: String,
}

#[derive(Deserialize, Debug)]
struct GoogleJwkKeys {
    keys: Vec<Jwk>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    sub: String,
    name: String,
    email: String,
}

async fn validate_token(
    GoogleCredential { credential }: GoogleCredential,
) -> Result<UserInfo, (StatusCode, String)> {
    // TODO: Caching.
    let google_jwks: GoogleJwkKeys = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
        .await
        .map_err(internal_error)?
        .json()
        .await
        .map_err(internal_error)?;
    // XXX: For now [1] seems to work and [0] gives `InvalidSignature`. This may change in future
    // in which case the solution will be to cycle through the keys until one of them works.
    let jwk = &google_jwks.keys[1];
    let mut validation = Validation::new(Algorithm::RS256);
    validation
        .set_audience(&[&env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set")]);
    validation.set_issuer(&["accounts.google.com", "https://accounts.google.com"]);

    Ok(jsonwebtoken::decode::<UserInfo>(
        &credential,
        &DecodingKey::from_jwk(jwk).map_err(internal_error)?,
        &validation,
    )
    .map_err(|err| (StatusCode::UNAUTHORIZED, err.to_string()))?
    .claims)
}

pub async fn login(
    Json(google_credential): Json<GoogleCredential>,
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    use crate::schema::users::*;
    let mut conn = establish_connection();
    let result = validate_token(google_credential).await?;
    diesel::insert_into(table)
        .values((
            id.eq(&result.sub),
            name.eq(&result.name),
            email.eq(&result.email),
        ))
        .on_conflict(id)
        .do_nothing()
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(result))
}
