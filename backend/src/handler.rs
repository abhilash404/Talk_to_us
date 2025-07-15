use sea_orm::{Database, DatabaseConnection, EntityTrait,QueryFilter,QuerySelect,ColumnTrait};
use chrono::{Utc, TimeZone};

use axum::{
    extract::{Path,State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::entities::{tickets,replies};
use crate::login::{require_role,AuthenticatedUser};

pub async fn get_tickets(
    State(db): State<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    match tickets::Entity::find().all(&*db).await {
        Ok(ticket_list) => Json(ticket_list).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        ).into_response(),
    }
}


#[derive(serde::Serialize)]
struct TicketSummary {
    title: String,
    priority: String,
    status: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}
pub async fn get_user_ticket_summaries(
    AuthenticatedUser(user): AuthenticatedUser,
    Path(user_id): Path<i32>,
    State(db): State<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    // Allow both "admin" and "agent" roles
    if let Err(e) = require_role(&AuthenticatedUser(user), &["admin", "agent"]) {
        return e.into_response();
    }
    match tickets::Entity::find()
        .filter(tickets::Column::CreatedBy.eq(user_id))
        .all(&*db)
        .await
    {
        Ok(ticket_list) => {
            let summaries: Vec<TicketSummary> = ticket_list
                .into_iter()
                .map(|t| TicketSummary {
                    title: t.title,
                    priority: t.priority.unwrap_or_else(|| "normal".to_string()),
                    status: t.status,
                    created_at: t.created_at.map(|naive| chrono::Utc.from_utc_datetime(&naive)),
                })
                .collect();
            Json(summaries).into_response()
        }
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        ).into_response(),
    }
}


pub async fn open_tickets(Path(user_id): Path<i32>,
    State(db): State<Arc<DatabaseConnection>>,) {

}

pub async fn resolved(
    AuthenticatedUser(user): AuthenticatedUser,
    Path(user_id): Path<i32>,
    State(db): State<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    // Only allow access for "admin" and "agent" roles
    if let Err(e) = require_role(&AuthenticatedUser(user), &["admin", "agent"]) {
        return e.into_response();
    }
    match tickets::Entity::find()
        .filter(tickets::Column::CreatedBy.eq(user_id))
        .filter(tickets::Column::Status.eq("closed"))
        .all(&*db)
        .await
    {
        Ok(ticket_list) => {
            let summaries: Vec<TicketSummary> = ticket_list
                .into_iter()
                .map(|t| TicketSummary {
                    title: t.title,
                    priority: t.priority.unwrap_or_else(|| "normal".to_string()),
                    status: t.status,
                    created_at: t.created_at.map(|naive| Utc.from_utc_datetime(&naive)),
                })
                .collect();
            Json(summaries).into_response()
        }
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        ).into_response(),
    }
}