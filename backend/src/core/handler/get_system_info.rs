use ipc::action::ActionPayload;
use wry::application::{event_loop::EventLoopProxy, window::WindowId};

use crate::{api, core::event::UserEvent};

use std::sync::Arc;

use crate::ipc::{self, CallbackPayload};

pub fn handle(
    payload: ActionPayload,
    window_id: WindowId,
    event_proxy: Arc<EventLoopProxy<UserEvent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(callback_id) = payload.callback_id {
        event_proxy
            .send_event(UserEvent::Callback(
                window_id,
                CallbackPayload::with_ended(callback_id),
                api::system::get_system_info()?,
            ))
            .ok();
    };
    Ok(())
}
