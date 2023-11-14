use errors::app_erro::AppError;
use errors::erros::Errors;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;

pub struct FakeEmail
{
    to: String,
    subject: String,
    body: String
}

impl FakeEmail 
{
    pub fn new(to: String, subject: String, body: String) -> Self
    {
        Self { to: to, subject: subject, body: body }
    }
    pub fn dispatch_email(self) -> Result<(), AppError>
    {
        dotenv::dotenv()
        .map_err(|_: dotenv::Error| AppError { error_code: Errors::NOT_FOUND })?;

        let smtp_key: String = dotenv::var("SMTP_PASSWORD")
        .map_err(|_: dotenv::Error| AppError { error_code: Errors::FAILED_TO_READ_FILE })?;
        let from_email: String = dotenv::var("SMTP_USERNAME")
        .map_err(|_: dotenv::Error| AppError { error_code: Errors::FAILED_TO_READ_FILE })?;
        let to_email: String = self.to;
        let host: String = dotenv::var("SMTP_HOST")
        .map_err(|_: dotenv::Error| AppError { error_code: Errors::FAILED_TO_READ_FILE })?;


        let email: Message = Message::builder()
        .header(ContentType::TEXT_HTML)
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject(self.subject)
        .body(self.body)
        .unwrap();

        let creds: Credentials = Credentials::new(from_email.to_string(), smtp_key.to_string());

        let mailer: SmtpTransport = SmtpTransport::relay(&host)
        .map_err(|_: lettre::transport::smtp::Error| AppError { error_code: Errors::INTERNAL_SERVER_ERROR })?
        .credentials(creds)
        .build();

        match mailer.send(&email) 
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError { error_code: Errors::INTERNAL_SERVER_ERROR })
        }
    }
}