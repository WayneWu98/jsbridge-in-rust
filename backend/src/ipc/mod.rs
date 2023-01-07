pub mod action;
pub mod event;

use serde::Serialize;
use serde_json::json;
use std::error::Error;
use wry::webview::WebView;

pub struct CallbackPayload(isize, bool);

impl CallbackPayload {
    pub fn new(callback_id: isize, call_ended: bool) -> Self {
        CallbackPayload(callback_id, call_ended)
    }
    pub fn with_ended(callback_id: isize) -> Self {
        Self::new(callback_id, true)
    }
    pub fn with_unended(callback_id: isize) -> Self {
        Self::new(callback_id, false)
    }
}

pub fn callback(
    wv: &WebView,
    CallbackPayload(callback_id, call_ended): CallbackPayload,
    data: Option<impl Serialize>,
) -> Result<(), Box<dyn Error>> {
    wv.evaluate_script(&format!(
        "window.onReceivedMsg({})",
        json!({
            "callbackId": callback_id,
            "callEnded": serde_json::Value::Bool(call_ended),
            "data": data,
            "timestamp": chrono::Utc::now().timestamp_millis(),
        })
    ))?;

    Ok(())
}

pub fn notice(
    wv: &WebView,
    event: event::Event,
    data: Option<impl Serialize>,
) -> Result<(), Box<dyn Error>> {
    wv.evaluate_script(&format!(
        "window.onReceivedMsg({})",
        json!({
            "event": event,
            "data": data,
            "timestamp": chrono::Utc::now().timestamp_millis(),
        })
    ))?;

    Ok(())
}
