use axum::{http::{StatusCode, header}, response::IntoResponse, Json};
use serde_json::json;
use crate::erros;

#[derive(Debug)]
pub struct ApiError 
{
    pub error_code: erros::Errors,
    pub status_code: StatusCode,
    pub custom_message: Option<String>
}

impl IntoResponse for ApiError 
{
    fn into_response(self) -> axum::response::Response 
    {
        let error_code = self.error_code;
        match self.custom_message 
        {
            Some(m) => (self.status_code, [(header::CONTENT_TYPE, "application/json")], 
            Json(json!(
                {
                    "ErrorCode": u16::from(error_code.clone()), 
                    "StatusCode": u16::from(self.status_code), 
                    "Message": format!("{}: {}", String::from(error_code.clone().message().ok_or("Internal Server Error").unwrap()), m)
                }))).into_response(),
            None => (self.status_code, [(header::CONTENT_TYPE, "application/json")], 
            Json(json!(
                {
                    "ErrorCode": u16::from(error_code.clone()), 
                    "StatusCode": u16::from(self.status_code), 
                    "Message": error_code.message().clone()
                }))).into_response()
        }
        
    }    
}