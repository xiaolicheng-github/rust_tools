// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod system_info;
mod net_received_transmitted;
mod http_service;
use std::sync::Mutex;

struct AppState {
    http_service: Mutex<Option<http_service::HttpService>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_system_info() -> system_info::Info {
    return system_info::get_system_info();
}

#[tauri::command]
fn get_net_received_transmitted() -> net_received_transmitted::NetReceivedTransmitted {
    return net_received_transmitted::get_net_received_transmitted();
}

#[tauri::command]
async fn start_http_service(port: u16, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut service = http_service::HttpService::new(port);
    let result = service.start().await;
    *state.http_service.lock().map_err(|e| e.to_string())? = Some(service);
    Ok(result)
}

#[tauri::command]
async fn stop_http_service(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let service_opt = state.http_service.lock()
        .map_err(|e| format!("状态锁获取失败: {}", e))?
        .take();
    println!("fasdfsadf");
    match service_opt {
        Some(mut service) => Ok(service.stop().await),
        None => Ok("HTTP服务未运行".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            http_service: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_system_info,
            get_net_received_transmitted,
            start_http_service,
            stop_http_service
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
