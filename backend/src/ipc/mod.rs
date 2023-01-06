pub mod action;
pub mod event;
pub mod system;

use action::{Action, ActionPayload};
use std::error::Error;
use wry::webview::WebView;

pub struct CallbackPayload(isize, bool);

impl CallbackPayload {
    fn new(callback_id: isize, call_ended: bool) -> Self {
        CallbackPayload(callback_id, call_ended)
    }
    fn with_ended(callback_id: isize) -> Self {
        Self::new(callback_id, true)
    }
    fn with_unended(callback_id: isize) -> Self {
        Self::new(callback_id, false)
    }
}

pub fn callback(
    wv: &WebView,
    CallbackPayload(callback_id, call_ended): CallbackPayload,
    data: Option<serde_json::Value>,
) -> Result<(), Box<dyn Error>> {
    wv.evaluate_script(&format!(
        "window.onReceivedMsg({{ callbackId: {}, callEnded: {}, data: {} }})",
        callback_id,
        serde_json::Value::Bool(call_ended),
        if let Some(data) = data {
            data
        } else {
            serde_json::Value::Null
        }
    ))?;

    Ok(())
}

pub fn notice(
    wv: &WebView,
    event: event::Event,
    data: Option<serde_json::Value>,
) -> Result<(), Box<dyn Error>> {
    wv.evaluate_script(&format!(
        "window.onReceivedMsg({{ event: \"{}\", data: {:} }})",
        event,
        if let Some(data) = data {
            data
        } else {
            serde_json::Value::Null
        }
    ))?;
    Ok(())
}

pub fn handle_ipc_msg(msg: String, wv: &WebView) -> Result<(), Box<dyn Error>> {
    let payload: ActionPayload = serde_json::from_str(&msg)?;
    match payload.action_type {
        Action::GetSystemInfo => {
            if let Some(callback_id) = payload.callback_id {
                callback(
                    wv,
                    CallbackPayload::with_ended(callback_id),
                    system::get_system_info()?,
                )
                .ok();
            }
        }
        _ => (),
    };
    Ok(())
}
