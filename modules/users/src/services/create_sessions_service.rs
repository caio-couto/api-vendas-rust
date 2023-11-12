use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::{ DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, DbErr};
use serde::{Deserialize, Serialize};
use crate::entity::users::{ Entity as Users, self, Model};
use bcrypt::{ verify, BcryptError};
use jsonwebtoken::{encode, EncodingKey, Header};
use config::auth::JWT;

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct CreateSessionDto 
{
    pub email: String, 
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateSessionsClaims
{
    subject: String,
    exp: usize
}

#[derive(Serialize)]
pub struct CreateSessionResponse
{
    user: Model,
    token: String
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct CreateSessionsService 
{
    connection: Arc<DatabaseConnection>,
}

impl CreateSessionsService 
{
    pub async fn execute(&self, create_session_dto: CreateSessionDto) -> Result<CreateSessionResponse, ApiError>
    {
        let user = Users::find()
        .filter(users::Column::Email.eq(&create_session_dto.email))
        .one(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError {error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None})?;

        let user = match user 
        {
            Some(u) => u,
            None => return Err(ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::UNAUTHORIZED, custom_message: Some("Incorrect Email/Passwod Combination.".to_string()) }),
        };

        let password = verify(create_session_dto.password, &user.password)
        .map_err(|_: BcryptError| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?; 

        if !password 
        {
            return Err(ApiError { error_code: Errors::INVALIDE_CREDENTIALS, status_code: StatusCode::UNAUTHORIZED, custom_message: Some("Incorrect Email/Passwod Combination.".to_string()) });
        }

        let jwt_config = JWT::new().unwrap();
        let jwt_payload = CreateSessionsClaims {subject: user.id.clone().to_string(), exp: jwt_config.exp};

        let token = match encode(&Header::default(), &jwt_payload, &EncodingKey::from_secret(&jwt_config.secret.as_bytes()))
        .map_err(|_: jsonwebtoken::errors::Error| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })
        {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        
        Ok(CreateSessionResponse { user: user, token: token })
    }    
}
