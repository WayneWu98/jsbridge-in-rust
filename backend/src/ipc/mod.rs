pub mod action;
pub mod event;
pub mod system;

use action::{Action, ActionPayload};
use std::error::Error;
use wry::webview::WebView;

pub fn callback(
    wv: &WebView,
    callback_id: Option<isize>,
    data: Option<serde_json::Value>,
) -> Result<(), Box<dyn Error>> {
    if let Some(callback_id) = callback_id {
        wv.evaluate_script(&format!(
            "window.onReceivedMsg({{ callbackId: {}, data: {:} }})",
            callback_id,
            if let Some(data) = data {
                data
            } else {
                serde_json::Value::Null
            }
        ))?;
    }

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
            system::get_system_info(wv, payload)?;
        }
        _ => (),
    };
    Ok(())
}
