mod entities;
mod login;
mod ticket;
mod handler;
use sea_orm::{Database, EntityTrait};

use axum::{
    routing::{get,post},
    Router,http
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use http::HeaderValue;


use handler::{get_tickets,get_user_ticket_summaries,open_tickets,resolved};
use login::{login_handler,register_handler};
use ticket::{create_ticket,reply_ticket};

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let db = Arc::new(Database::connect("postgres://postgres:%40bhil%40%24h12@localhost:5432/Talk").await?);
    let cors = CorsLayer::new()
    .allow_origin("http://127.0.0.1:5501".parse::<HeaderValue>().unwrap()) 
    .allow_methods(Any)
    .allow_headers(Any);

    let app = Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
        .route("/create", post(create_ticket))
        .route("/reply", post(reply_ticket))
        .route("/tickets", get(get_tickets))
        .route("/tickets/:user_id",get(get_user_ticket_summaries))
        .route("/tickets/:user_id/open",get(open_tickets))
        .route("/tickets/:user_id/resolved",get(resolved))
        .with_state(db.clone())
        .layer(cors); // Attach db to state

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
