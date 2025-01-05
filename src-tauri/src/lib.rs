// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod system_info;
mod net_received_transmitted;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_system_info,
            get_net_received_transmitted
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
