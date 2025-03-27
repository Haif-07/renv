// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::Mutex;
use crate::config_file::ShellConfig;

mod msg; // 引入同级模块 msg.rs
mod config_file; // 引入配置文件模块
mod commands; // 引入命令模块

#[derive(Default)]
pub struct AppState {
    shell_config: Mutex<Option<ShellConfig>>
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState::default();
    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_config_files,
            commands::load_config_file_content,
            commands::save_config_file,
            commands::delete_env_variable,
            commands::add_env_variable,
            commands::update_env_variable,
            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
