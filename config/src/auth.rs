use errors::{app_erro::AppError, erros::Errors};
use chrono::{Local, Days};

pub struct JWT
{
    pub secret: String,
    pub exp: usize
}

impl JWT 
{
    pub fn new() -> Result<Self, AppError>
    {
        dotenv::dotenv()
        .map_err(|_: dotenv::Error| AppError { error_code: Errors::NOT_FOUND })?;
        
        let secret = dotenv::var("APP_SECRET")
        .map_err(|_: dotenv::Error| AppError { error_code: Errors::FAILED_TO_READ_FILE })?;

        let exp_timestamp = Local::now().checked_add_days(Days::new(1)).unwrap().timestamp() as usize;

        Ok(Self { secret: secret, exp: exp_timestamp })
    }    
}