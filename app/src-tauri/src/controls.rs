// In src-tauri/src/controls.rs

use crate::AppState;
use axum::{extract::State as AxumState, http::StatusCode};
use enigo::{Button, Enigo, Mouse, Settings};

/// Handler for /click endpoint
/// Triggers a mouse click at the current cursor position
pub async fn click_handler(AxumState(_state): AxumState<AppState>) -> StatusCode {
    log::info!("Received click request");

    match Enigo::new(&Settings::default()) {
        Ok(mut enigo) => match enigo.button(Button::Left, enigo::Direction::Click) {
            Ok(_) => {
                log::info!("Mouse click executed successfully");
                StatusCode::OK
            }
            Err(e) => {
                log::error!("Failed to execute mouse click: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        },
        Err(e) => {
            log::error!("Failed to initialize Enigo: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
