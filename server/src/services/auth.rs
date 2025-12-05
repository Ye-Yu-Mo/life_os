use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::entities::{prelude::*, user};
use crate::errors::AuthError;

pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
}

pub async fn register(
    db: &DatabaseConnection,
    req: RegisterRequest,
) -> Result<UserResponse, AuthError> {
    let username = req.username.trim().to_lowercase();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AuthError::PasswordHashError)?
        .to_string();

    let user_id = Uuid::new_v4();
    let new_user = user::ActiveModel {
        id: Set(user_id),
        username: Set(username.clone()),
        password_hash: Set(password_hash),
        created_at: Set(Utc::now().into()),
    };

    match new_user.insert(db).await {
        Ok(_) => Ok(UserResponse {
            id: user_id,
            username,
        }),
        Err(sea_orm::DbErr::Exec(_)) | Err(sea_orm::DbErr::Query(_)) => {
            Err(AuthError::RegistrationFailed)
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn login(
    db: &DatabaseConnection,
    req: LoginRequest,
) -> Result<UserResponse, AuthError> {
    let username = req.username.trim().to_lowercase();

    let user = User::find()
        .filter(user::Column::Username.eq(&username))
        .one(db)
        .await?
        .ok_or(AuthError::AuthenticationFailed)?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AuthError::AuthenticationFailed)?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AuthError::AuthenticationFailed)?;

    Ok(UserResponse {
        id: user.id,
        username: user.username,
    })
}
