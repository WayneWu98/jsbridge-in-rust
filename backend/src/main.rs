use std::cell::RefCell;
use std::collections::HashMap;

use backend::ipc::{self, notice};
use wry::application::window::{Theme, WindowId};
use wry::webview::WebView;
use wry::{
    application::{
        dpi::{PhysicalPosition, PhysicalSize, Position, Size},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

thread_local! {
    static WEBVIEWS: RefCell<HashMap<WindowId, WebView>> = RefCell::new(HashMap::new());
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("An Implementation of JsBridge")
        .with_inner_size(Size::Physical(PhysicalSize::new(750, 1334)))
        .with_position(Position::Physical(PhysicalPosition::new(2400, 400)))
        .build(&event_loop)?;
    let id = window.id().clone();
    let webview = WebViewBuilder::new(window)?
        .with_url("http://127.0.0.1:5173/")?
        .with_ipc_handler(move |window, msg| {
            WEBVIEWS.with(move |webviews| {
                if let Some(wv) = webviews.borrow().get(&window.id()) {
                    ipc::handle_ipc_msg(msg, &wv).unwrap();
                }
            })
        })
        .build()?;
    webview.open_devtools();
    WEBVIEWS.with(move |webviews| {
        webviews.borrow_mut().insert(id, webview);
    });
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::ThemeChanged(theme),
                ..
            } => {
                WEBVIEWS.with(|webviews| {
                    for (_, webview) in webviews.borrow().iter() {
                        notice(
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
                });
            }
            _ => (),
        }
    });
}
