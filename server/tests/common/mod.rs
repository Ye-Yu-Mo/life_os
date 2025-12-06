use chrono::Utc;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Set};
use server::entities::{prelude::*, user};
use uuid::Uuid;

pub async fn setup_test_db() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/life_os".to_string());

    Database::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

pub async fn create_test_user(db: &DatabaseConnection) -> Uuid {
    let user_id = Uuid::new_v4();
    let now = Utc::now().into();

    let user = user::ActiveModel {
        id: Set(user_id),
        username: Set(format!("test_user_{}", user_id)),
        password_hash: Set("dummy_hash".to_string()),
        created_at: Set(now),
    };

    user.insert(db)
        .await
        .expect("Failed to create test user");

    user_id
}

pub async fn cleanup_test_user(db: &DatabaseConnection, user_id: Uuid) {
    if let Ok(user) = User::find_by_id(user_id).one(db).await {
        if let Some(user) = user {
            let active: user::ActiveModel = user.into();
            let _ = active.delete(db).await;
        }
    }
}
