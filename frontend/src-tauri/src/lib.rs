use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub os: String,
}

#[tauri::command]
fn get_app_info() -> AppInfo {
    AppInfo {
        name: "EpicTask".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        os: std::env::consts::OS.to_string(),
    }
}

#[tauri::command]
fn send_desktop_notification(title: String, body: String) -> Result<String, String> {
    println!("[Desktop Notification] {}: {}", title, body);
    Ok("Notification sent".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            send_desktop_notification
        ])
        .run(tauri::generate_context!())
        .expect("Error while running EpicTask Tauri desktop application");
}
