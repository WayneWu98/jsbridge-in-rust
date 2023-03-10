use ipc::action::{Action, ActionPayload};
use std::error::Error;
use wry::{
    application::{
        event_loop::EventLoopProxy,
        window::{Theme, WindowId},
    },
    webview::WebView,
};

use std::{collections::HashMap, sync::Arc, time::Duration};

use sysinfo::{CpuExt, SystemExt};
use wry::application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::ControlFlow,
};

use crate::ipc::{self, CallbackPayload};

pub enum UserEvent {
    Notice(ipc::event::Event, Option<serde_json::Value>),
    Callback(WindowId, CallbackPayload, Option<serde_json::Value>),
}

pub fn handle_event(
    event: Event<UserEvent>,
    control_flow: &mut ControlFlow,
    webviews: &HashMap<WindowId, WebView>,
) -> Result<(), Box<dyn std::error::Error>> {
    match event {
        Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        Event::UserEvent(user_event) => match user_event {
            UserEvent::Notice(event, data) => {
                for (_, webview) in webviews.iter() {
                    ipc::notice(webview, event, data.clone()).unwrap();
                }
            }
            UserEvent::Callback(window_id, callback_payload, data) => {
                if let Some(webview) = webviews.get(&window_id) {
                    ipc::callback(webview, callback_payload, data).ok();
                }
            }
        },
        Event::WindowEvent {
            event: WindowEvent::ThemeChanged(theme),
            ..
        } => {
            for (_, webview) in webviews.iter() {
                ipc::notice(
                    webview,
                    ipc::event::Event::ThemeChanged,
                    Some(serde_json::Value::String(if let Theme::Dark = theme {
                        "dark".to_owned()
                    } else {
                        "light".to_owned()
                    })),
                )
                .unwrap();
            }
        }
        _ => (),
    }
    Ok(())
}

pub fn handle_ipc_msg(
    msg: String,
    window_id: WindowId,
    event_proxy: Arc<EventLoopProxy<UserEvent>>,
) -> Result<(), Box<dyn Error>> {
    if let Ok(payload) = serde_json::from_str::<ActionPayload>(&msg) {
        return match payload.action_type {
            Action::GetSystemInfo => {
                crate::core::handler::get_system_info::handle(payload, window_id, event_proxy)
            }
            Action::DownloadFile => {
                crate::core::handler::download::handle(payload, window_id, event_proxy)
            }
            _ => Ok(()),
        };
    }
    Ok(())
}
