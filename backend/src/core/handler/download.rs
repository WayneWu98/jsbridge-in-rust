use ipc::action::ActionPayload;
use serde_json::json;
use wry::application::{event_loop::EventLoopProxy, window::WindowId};

use crate::{core::event::UserEvent, utils::download};

use std::sync::Arc;

use crate::ipc::{self, CallbackPayload};

#[derive(serde::Serialize, Clone)]
enum CallbackType {
    Progress(u64, u64),
    Success,
    Fail(String),
    Finally,
}

impl CallbackType {
    fn to_json(self) -> serde_json::Value {
        match self {
            CallbackType::Progress(progress, total) => {
                json!({ "type": "Progress", "data": { "total": total.clone(), "progress": progress.clone() } })
            }
            CallbackType::Fail(err) => {
                json!({ "type": "Fail", "data": err })
            }
            _ => json!({ "type": self }),
        }
    }
}

pub fn handle(
    payload: ActionPayload,
    window_id: WindowId,
    event_proxy: Arc<EventLoopProxy<UserEvent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(data) = payload.data {
        let url = data.as_str().unwrap_or("").to_owned();
        let callback_id = payload.callback_id.unwrap_or(0);
        let id = window_id.clone();
        let send_event = move |callback_type| {
            let cb_payload = if let CallbackType::Finally = callback_type {
                CallbackPayload::with_ended(callback_id)
            } else {
                CallbackPayload::with_unended(callback_id)
            };
            event_proxy.send_event(UserEvent::Callback(
                id.clone(),
                cb_payload,
                Some(callback_type.to_json()),
            ))
        };
        tokio::spawn(async move {
            let err = download(url, |progress, total| {
                send_event(CallbackType::Progress(progress, total)).ok();
            })
            .await
            .err();
            if let Some(err) = err {
                send_event(CallbackType::Fail(err.to_string())).ok();
            } else {
                send_event(CallbackType::Success).ok();
            }
            send_event(CallbackType::Finally).ok();
        });
    }
    Ok(())
}
