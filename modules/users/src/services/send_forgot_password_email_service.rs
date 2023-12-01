use std::{sync::Arc, ops::Deref};
use axum::http::StatusCode;
use derive_builder::Builder;
use errors::{api_erro::ApiError, erros::Errors};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, DbErr, ActiveModelTrait};
use serde::Deserialize;
use crate::entity::users::{ Entity as Users, self};
use crate::entity::user_tokens;
use config::mail::{mailtutan_mail::FakeEmail, mail_template::MailTemplate};

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
#[derive(Deserialize)]
pub struct SendForgotPasswordEmailDto 
{
    pub email: String, 
}

#[derive(Builder, Clone, Default, Debug)]
#[builder(setter(into))]
pub struct SendForgotPasswordEmailService 
{
    connection: Arc<DatabaseConnection>,
}

impl SendForgotPasswordEmailService 
{
    pub async fn execute(&self, send_forgot_password_email_dto: SendForgotPasswordEmailDto) -> Result<(), ApiError>
    {
        dotenv::dotenv()
        .map_err(|_| ApiError {status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code: Errors::INTERNAL_SERVER_ERROR, custom_message: None})?;

        let user = Users::find()
        .filter(users::Column::Email.eq(send_forgot_password_email_dto.email))
        .one(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        let user = user.ok_or(ApiError { error_code: Errors::USER_NOT_FOUND, status_code: StatusCode::NOT_FOUND, custom_message: Some("Token not found".to_string()) })?;
 
        let token = user_tokens::ActiveModel
        {
            id: NotSet,
            token: NotSet,
            user_id: Set(user.id),
            created_at: NotSet,
            updated_at: NotSet
        };

        let token: user_tokens::Model = token.insert(self.connection.deref())
        .await
        .map_err(|_: DbErr| ApiError { error_code: Errors::INTERNAL_SERVER_ERROR, status_code: StatusCode::INTERNAL_SERVER_ERROR, custom_message: None })?;

        let url: String = dotenv::var("APP_WEB_URL")
        .map_err(|_| ApiError {status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code: Errors::INTERNAL_SERVER_ERROR, custom_message: None})?;

        let body = MailTemplate::forgot_password_template(user.name, format!("{}/reset_password?token={}", url, token.token));

        let fake_email = FakeEmail::new(user.email, "Forgot Password Token".to_string(), body);  

        let _ = fake_email.dispatch_email();

        Ok(())
    }    
}
