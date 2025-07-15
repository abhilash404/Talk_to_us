use http::response;
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



#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>, // e.g., "low", "medium", "high"
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Reply{
  pub reply_text: String,
  pub ticket_id : i32,
}

use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::{StatusCode},
};

use crate::entities::{tickets,replies};
use crate::login::{AuthenticatedUser, require_role};

pub async fn create_ticket(
    AuthenticatedUser(user): AuthenticatedUser,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(payload): Json<CreateTicketRequest>,
) -> impl IntoResponse {
    
    if let Err(e) = require_role(&AuthenticatedUser(user.clone()), &["customer"]) {
        return e.into_response();
    }

    // Create ticket model
    let new_ticket = tickets::ActiveModel {
    title: Set(payload.title),
    description: Set(payload.description.unwrap_or_default()),
    priority: Set(Some(payload.priority.unwrap_or_else(|| "normal".to_string()))),
    status: Set("open".to_string()),
    created_by: Set(user.sub),
    created_at: Set(Some(Utc::now().naive_utc())),
    ..Default::default()
};


    
    match new_ticket.insert(&*db).await {
        Ok(saved_ticket) => {
            (StatusCode::CREATED, Json(saved_ticket)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create ticket: {}", e),
        ).into_response(),
    }
}


pub async fn reply_ticket(
    AuthenticatedUser(user): AuthenticatedUser,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(payload): Json<Reply>,
) -> impl axum::response::IntoResponse {
    if let Err(e) = require_role(&AuthenticatedUser(user.clone()), &["agent", "admin"]) {
        return e.into_response();
    }

    let now_fixed: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();

    let new_reply = replies::ActiveModel {
        ticket_id: Set(payload.ticket_id),
        agent_id: Set(Some(user.sub)),
        reply_text: Set(payload.reply_text),
        created_at: Set(Some(now_fixed)),
        ..Default::default()
    };

    match new_reply.insert(&*db).await {
    Ok(saved_reply) => (StatusCode::CREATED, Json(saved_reply)).into_response(),
    Err(e) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": format!("Failed to create reply: {}", e)}))
    ).into_response(),
    }
}

