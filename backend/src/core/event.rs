use std::{collections::HashMap, sync::Arc, time::Duration};

use sysinfo::{Cpu, CpuExt, SystemExt};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoopProxy},
        window::{Theme, WindowId},
    },
    webview::WebView,
};

use crate::ipc;

pub struct UserEvent(ipc::event::Event, serde_json::Value);

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
        Event::UserEvent(UserEvent(event, data)) => {
            for (_, webview) in webviews.iter() {
                ipc::notice(webview, event, Some(data.clone())).unwrap();
            }
        }
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

pub fn listen(
    event_proxy: Arc<EventLoopProxy<UserEvent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        tokio::join!(listen_cpu_changed(|system| {
            let data: Vec<serde_json::Value> = system
                .cpus()
                .iter()
                .map(|cpu| {
                    serde_json::json!({
                        "name": cpu.name(),
                        "frequency": cpu.frequency(),
                        "usage": cpu.cpu_usage(),
                        "vendorId": cpu.vendor_id(),
                        "brand": cpu.brand(),
                    })
                })
                .collect();

            event_proxy
                .send_event(UserEvent(
                    ipc::event::Event::CPUChanged,
                    serde_json::Value::Array(data),
                ))
                .ok();
            false
        }))
    });
    Ok(())
}

pub async fn listen_cpu_changed(f: impl Fn(sysinfo::System) -> bool) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        if f(sysinfo::System::new_all()) {
            break;
        }
    }
}
