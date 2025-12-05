use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::error;

pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await.map_err(|err| {
        error!(error = %err, "database connection failed");
        err
    })
}
