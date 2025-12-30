use crate::socket::types::WsMessage;
use actix_web::{ web, HttpRequest, Responder };
use actix_ws::Message;
use futures_util::StreamExt;
use serde_json::json;
use crate::socket::registry::WsRegistry;
use crate::state::AppState;
use crate::utils::auth::validate_and_refresh_session;

pub async fn ws_route(
    req: HttpRequest,
    body: web::Payload,
    registry: web::Data<WsRegistry>,
    app_state: web::Data<AppState>
) -> Result<impl Responder, actix_web::Error> {
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;
    let _ = session.text(
        json!({
        "type": "CONNECTION",
        "success": true,
        "subtype": "CONNECTED",
        "data": "WebSocket connection established. Please IDENTIFY."
    }).to_string()
    ).await;

    // Spawn a task to process incoming messages
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(parsed) = serde_json::from_str::<WsMessage>(&text) {
                        match parsed {
                            WsMessage::CONNECTION { subtype, data, .. } => {
                                match subtype.as_str() {
                                    "IDENTIFY" => {
                                        if
                                            let Some(token) = data
                                                .get("token")
                                                .and_then(|t| t.as_str())
                                        {
                                            match
                                                validate_and_refresh_session(
                                                    token.to_string(),
                                                    &app_state
                                                ).await
                                            {
                                                Ok(session_data) => {
                                                    let uid = session_data.user_uuid.to_string();

                                                    registry
                                                        .lock()
                                                        .unwrap()
                                                        .insert(uid.clone(), session.clone());

                                                    let _ = session.text(
                                                        json!({
                                                        "type": "CONNECTION",
                                                        "success": true,
                                                        "subtype": "IDENTIFY",
                                                        "data": { "success": "Identified.", "user": uid }
                                                    }).to_string()
                                                    ).await;
                                                }
                                                Err(_) => {
                                                    let _ = session.text(
                                                        json!({
                                                        "type": "ERROR",
                                                        "success": false,
                                                        "subtype": "IDENTIFY",
                                                        "data": { "error": "Invalid token." }
                                                    }).to_string()
                                                    ).await;
                                                }
                                            }
                                        }
                                    }
                                    "KEEPALIVE" => {
                                        let _ = session.text(
                                            json!({
                                            "type": "CONNECTION",
                                            "success": true,
                                            "subtype": "KEEPALIVE",
                                            "data": "Keepalive received."
                                        }).to_string()
                                        ).await;
                                    }
                                    _ => {}
                                }
                            }
                            WsMessage::ANNOUNCEMENT { subtype, .. } => {
                                if subtype == "GETANNOUNCEMENTS" {
                                    let anns =
                                        app_state.services.announcement_service.get_announcements().await;
                                    let _ = session.text(
                                        json!({
                                        "type": "ANNOUNCEMENT",
                                        "success": true,
                                        "subtype": "GETANNOUNCEMENTS",
                                        "data": anns
                                    }).to_string()
                                    ).await;
                                }
                            }
                        }
                    } else {
                        let _ = session.text(
                            json!({
                            "type": "ERROR",
                            "success": false,
                            "data": { "error": "Invalid JSON" }
                        }).to_string()
                        ).await;
                    }
                }
                Message::Ping(bytes) => {
                    let _ = session.pong(&bytes).await;
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}
