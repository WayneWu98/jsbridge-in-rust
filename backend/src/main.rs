use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use backend::core;
use backend::ipc;
use wry::application::window::WindowId;
use wry::webview::WebView;
use wry::{
    application::{
        dpi::{PhysicalPosition, PhysicalSize, Position, Size},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

thread_local! {
    static WEBVIEWS: RefCell<HashMap<WindowId, WebView>> = RefCell::new(HashMap::new());
}

#[tokio::main]
async fn main() -> wry::Result<()> {
    let event_loop = EventLoop::<core::event::UserEvent>::with_user_event();
    let event_proxy = Arc::new(event_loop.create_proxy());
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
    core::event::listen(event_proxy.clone()).unwrap();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        WEBVIEWS.with(|webviews| {
            match core::event::handle_event(event, control_flow, &webviews.borrow()) {
                Err(err) => {
                    println!("Some error fired: {}", err);
                }
                _ => {}
            };
        });
    });
}
