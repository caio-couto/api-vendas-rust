use axum::{http::{ Request, StatusCode, header::ToStrError}, middleware::Next, response::Response };
use errors::{api_erro::ApiError, erros::Errors};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use config::auth::JWT;
use sea_orm::prelude::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CreateSessionsClaims
{
    subject: String,
    exp: usize
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IsAuthenticatedMiddleware
{
    pub access_token_uuid: Uuid
}

pub async fn is_authenticated<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, ApiError>
{
    let header = &req.headers().get("authorization");

    let token = header.ok_or(ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::UNAUTHORIZED, custom_message: None})?;

    let token = token.to_str()
    .map_err(|_:ToStrError| ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::UNAUTHORIZED, custom_message: None})?;

    let token = token.split(' ').last()
    .ok_or(ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::UNAUTHORIZED, custom_message: None})?;

    let jwt_config = JWT::new().unwrap();

    let decoded_token = decode::<CreateSessionsClaims>(token, &DecodingKey::from_secret(&jwt_config.secret.as_bytes()), &Validation::new(Algorithm::HS256))
    .map_err(|err: jsonwebtoken::errors::Error| ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::UNAUTHORIZED, custom_message: Some(err.to_string())})?;

    let decoded_token = decoded_token.claims.subject;

    let id = Uuid::parse_str(decoded_token.as_str())
    .map_err(|_| ApiError { error_code: Errors::INVALID_UUID, status_code: StatusCode::BAD_REQUEST, custom_message: None })?;

    req.extensions_mut().insert(IsAuthenticatedMiddleware {access_token_uuid: id});

    Ok(next.run(req).await)
} 