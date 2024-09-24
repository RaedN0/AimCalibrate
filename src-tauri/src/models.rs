use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct YawStuff {
    pub sens: f64,
    pub counts: i32,
    pub inc: f64,
    pub yaw: f64,
    pub lower_limit: f64,
    pub upper_limit: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GameYaw {
    pub name: String,
    pub yaw: f64,
}

#[derive(Serialize, Clone)]
pub struct UserSettings {
    pub cm360: f64,
    pub dpi: i32,
    pub normal_fov: f64,
    pub scoped_fov: f64,
    pub game_sens: f64,
    pub game_fov: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub turn_speed: f32,
    pub hotkeys: Vec<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            turn_speed: 1.0,
            hotkeys: vec!["F1".to_string(), "F2".to_string(), "F3".to_string(), "F4".to_string()],
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct FovUpdatePayload {
    pub fov16: f64,
}
