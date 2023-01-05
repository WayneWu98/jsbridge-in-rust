fn main() -> wry::Result<()> {
    use wry::{
        application::{
            dpi::{PhysicalPosition, PhysicalSize, Position, Size},
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("An Implementation of JsBridge")
        .with_inner_size(Size::Physical(PhysicalSize::new(750, 1334)))
        .with_position(Position::Physical(PhysicalPosition::new(2400, 80)))
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_url("http://127.0.0.1:5173/")?
        .build()?;

    _webview.open_devtools();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
