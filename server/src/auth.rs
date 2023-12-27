use std::env;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{Json, Response},
};
use jsonwebtoken::{jwk::Jwk, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::internal_error;

/// Middleware for token authentication.
pub async fn auth(mut request: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    let credential = serde_json::from_str(
        request
            .headers()
            .get("Authorization: Bearer")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "no token found in headers".to_string(),
            ))?
            .to_str()
            .map_err(internal_error)?,
    )
    .map_err(internal_error)?;
    let user = validate_token(credential).await?;
    request
        .headers_mut()
        .insert("user_sub", user.sub.parse().unwrap());
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
    validate_token(google_credential).await.map(|res| Json(res))
}
