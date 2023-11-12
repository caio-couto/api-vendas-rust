use std::{error::Error, fmt::Display};
use crate::erros::Errors;

#[derive(Debug, Clone)]
pub struct AppError 
{
    pub error_code: Errors
}

impl Display for AppError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self.error_code.message() 
        {
          Some(m) => write!(f, "{}", m),
          None => write!(f, "Internal Server Error")    
        }
    }
}

impl Error  for AppError
{
    fn description(&self) -> &str 
    {
        &self.error_code.message().ok_or("Internal Server Error").unwrap()   
    }
}
