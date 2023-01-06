use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Action {
    GetSystemInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActionPayload {
    pub action_type: Action,
    pub callback_id: Option<isize>,
    pub data: Option<serde_json::Value>,
}
