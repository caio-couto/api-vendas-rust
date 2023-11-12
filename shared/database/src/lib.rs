use dotenv::Error;
use sea_orm::{ Database, DatabaseConnection, DbErr };
use errors::{ app_erro::AppError, erros::Errors};


pub async fn connection() -> Result<DatabaseConnection, AppError>
{
    dotenv::dotenv()
    .map_err(|_: Error|  AppError {error_code: Errors::NOT_FOUND})?;

    let database_url: String = dotenv::var("DATABASE_URL")
    .map_err(|_: Error| AppError {error_code: Errors::FAILED_TO_READ_FILE})?;

    let connection: DatabaseConnection = Database::connect(database_url)
    .await
    .map_err(|_: DbErr| AppError {error_code: Errors::INVALID_DATABASE_CONNECTION})?;

    let connection = connection;

    Ok(connection)
}
