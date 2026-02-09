//! Chat API Handlers
//!
//! Handles HTTP requests related to chat and upgrades connections to WebSocket.

use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use surrealdb::sql::Thing;

use crate::infrastructure::websocket::chat_server::ChatServer;
use crate::infrastructure::websocket::session::WsSession;

/// Handlers for WebSocket connection
///
/// Upgrades the HTTP connection to WebSocket and starts a WsSession actor.
/// For testing, we are using a dummy user_id.
pub async fn chat_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    // For now, we use a dummy user ID for testing.
    // In a real scenario, this would come from JWT claims.
    let dummy_user_id = Thing::from(("user", "test_user_id"));

    ws::start(
        WsSession::new(dummy_user_id, srv.get_ref().clone()),
        &req,
        stream,
    )
}
