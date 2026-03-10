use memori_ui::{layout::MemoriLayout, widgets::MemoriWidget};
use serde::Deserialize;

#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
#[specta(rename_all = "camelCase")]
pub struct MemoriStateInput {
    pub active_frame_idx: u32,
    pub widgets: Vec<MemoriWidget>,
    pub frames: Vec<MemoriLayout>,
    pub frame_time: u32,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthState {
    pub users_by_provider: ProviderUsers,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUsers {
    pub github: Option<AuthUserInfo>,
    pub twitch: Option<AuthUserInfo>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthUserInfo {
    pub id: String,
    pub name: String,
    pub access_token: String,
}

// Mirrored Auth Prefs Structs
#[derive(Debug, Deserialize, Default)]
pub struct Position {
    pub timestamp: i32,
    pub coords: Coordinates,
}

#[derive(Debug, Deserialize, Default)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
}

#[derive(Debug, Deserialize, Default)]
pub struct PrefsState {
    pub lastKnownLocation: Option<Position>,
    pub onboarded: bool,
    pub lastKnownDeviceId: Option<String>,
    pub name: String,
}
