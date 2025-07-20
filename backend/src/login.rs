use serde::{Deserialize, Serialize};
use jsonwebtoken::{EncodingKey, Header};
use sea_orm::{Database, DatabaseConnection, EntityTrait,QueryFilter,QuerySelect,ColumnTrait};
use chrono::{DateTime, Utc, TimeZone};
use axum::RequestPartsExt;
use jsonwebtoken::{decode,encode, DecodingKey, Validation, TokenData}; 
use std::sync::Arc;
use crate::entities::users;
use argon2::{Argon2, PasswordHash, PasswordVerifier, PasswordHasher};
use argon2::password_hash::SaltString;
use sea_orm::ActiveModelTrait;



use sea_orm::Set;
use axum::{
    extract::{Path,State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
};



#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Claims {
    pub sub: i32,      // user ID
    pub role: String,  // user role (e.g., "customer", "agent", "admin")
    pub exp: usize,    // expiration timestamp
}

pub struct AuthenticatedUser(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok());

        let Some(auth_header) = auth_header else {
            return Err((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()));
        };

        let token = auth_header.strip_prefix("Bearer ").ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization header".to_string(),
        ))?;

        let token_data: TokenData<Claims> = decode(
            token,
            &DecodingKey::from_secret("your-secret-key".as_ref()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(AuthenticatedUser(token_data.claims))
    }
}

pub fn require_role(user: &AuthenticatedUser, allowed_roles: &[&str]) -> Result<(), impl IntoResponse> {
    if allowed_roles.contains(&user.0.role.as_str()) {
        Ok(())
    } else {
        Err((StatusCode::FORBIDDEN, "Insufficient permissions"))
    }
}

pub async fn login_handler(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // 1. Find user by email
    let found_user = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&*db)
        .await;

    let user = match found_user {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(e) => {
            println!("DB error while checking user: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // 2. Verify password
    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(h) => h,
        Err(e) => {
            println!("PasswordHash parsing error: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Server error").into_response();
        }
    };
    let argon2 = Argon2::default();

    if argon2.verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    // 3. Create JWT with user id and role
    let claims = Claims {
        sub: user.id,
        role: user.role.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your-secret-key".as_ref()),
    ) {
        Ok(token) => token,
        Err(e) => {
            println!("JWT encode error: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed").into_response();
        }
    };

    let role = user.role;

    // 4. Return token and role
    (StatusCode::OK, Json(serde_json::json!({ "token": token, "role": role}))).into_response()
}



#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String, // e.g. "customer", "agent", "admin"
}


use rand_core::OsRng;

pub async fn register_handler(
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // Check if email already exists
    let found_user = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email.clone()))
        .one(&*db)
        .await;
    let user_exists = match found_user {
        Ok(user) => user.is_some(),
        Err(e) => {
            println!("DB error while checking user: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    if user_exists {
        return (StatusCode::CONFLICT, "Email already registered").into_response();
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
    {
        Ok(ph) => ph.to_string(),
        Err(e) => {
            println!("Password hash error: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Password hash error").into_response();
        }
    };

    // Save user to database
    let new_user = users::ActiveModel {
        username: Set(payload.username),
        email: Set(payload.email),
        password_hash: Set(password_hash),
        role: Set(payload.role.to_lowercase()), // convert to lowercase for consistency
        ..Default::default()
    };

    match new_user.insert(&*db).await {
        Ok(user) => {
            (StatusCode::CREATED, Json(serde_json::json!({
                "message": "User registered successfully",
                "user_id": user.id,
                "role": user.role,
            }))).into_response()
        }
        Err(e) => {
            println!("Failed to register user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to register user: {}", e))
                .into_response()
        }
    }
}