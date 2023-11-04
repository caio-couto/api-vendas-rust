use sea_orm::{ Database, DatabaseConnection };

pub async fn connection() -> anyhow::Result<DatabaseConnection>
{
    dotenv::dotenv()?;

    let database_url: String = dotenv::var("DATABASE_URL")?;

    let connection: DatabaseConnection = Database::connect(database_url).await?;

    let connection = connection;

    Ok(connection)
}
