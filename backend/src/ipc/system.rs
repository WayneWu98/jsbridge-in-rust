use serde_json::Map;
use serde_json::Value::{Null, Object, String};
use std::error::Error;
use sysinfo::{System, SystemExt};
use wry::webview::WebView;

use super::action::ActionPayload;

pub fn get_system_info(wv: &WebView, payload: ActionPayload) -> Result<(), Box<dyn Error>> {
    let sys = System::new_all();
    let mut map = Map::new();
    map.insert(
        "name".into(),
        if let Some(name) = sys.name() {
            String(name)
        } else {
            Null
        },
    );
    super::callback(wv, payload.callback_id, Some(Object(map)))?;
    Ok(())
}
