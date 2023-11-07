use axum::{http::{StatusCode, header}, response::IntoResponse, Json};
use serde_json::json;
pub mod erros;

#[derive(Debug)]
pub struct ApiError 
{
    pub error_code: erros::Errors,
    pub status_code: StatusCode,
}

impl IntoResponse for ApiError 
{
    fn into_response(self) -> axum::response::Response 
    {
        let error_code = self.error_code;
        (self.status_code, [(header::CONTENT_TYPE, "application/json")], Json(json!({"ErrorCode": u16::from(error_code.clone()), "StatusCode": u16::from(self.status_code), "Message": error_code.canonical_reason().clone()}))).into_response()
    }    
}