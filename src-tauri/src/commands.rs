use std::env;
use std::path::PathBuf;
use std::sync:: Mutex;
use tauri::State;

use crate::AppState;
use crate::config_file::{ShellConfig, EnvVariable};
use crate::msg::ApiResponse;



#[derive(serde::Serialize)]
pub struct ConfigFile {
    path: String,
    id: u8,
}

#[derive(serde::Serialize)]
pub struct EnvVariableResponse {
    index: u64,
    key: String,
    value: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct EnvVariableRequest {
    index: u64,
    key: String,
    value: Vec<String>,
}

/// 获取配置文件列表
#[tauri::command]
pub fn get_config_files() -> ApiResponse<Vec<ConfigFile>> {
    let home_dir = match env::var("HOME") {
        Ok(path) => PathBuf::from(path),
        Err(e) => return ApiResponse::error(500, "Failed to get home directory", &e.to_string()),
    };

    let config_files = vec![
        ConfigFile {
            path: home_dir.join(".zshenv").to_string_lossy().into_owned(),
            id: 1,
        },
        ConfigFile {
            path: home_dir.join(".zprofile").to_string_lossy().into_owned(),
            id: 2,
        },
        ConfigFile {
            path: home_dir.join(".zshrc").to_string_lossy().into_owned(),
            id: 3,
        },
        ConfigFile {
            path: home_dir.join(".zlogin").to_string_lossy().into_owned(),
            id: 4,
        },
        ConfigFile {
            path: home_dir.join(".zlogout").to_string_lossy().into_owned(),
            id: 5,
        },
    ];

    ApiResponse::success(config_files)
}

/// 加载配置文件内容
#[tauri::command]
pub fn load_config_file_content(state: State<AppState>, file_path: String) -> ApiResponse<Vec<EnvVariableResponse>> {
    let mut config = ShellConfig::new(&file_path);
    match config.load() {
        Ok(_) => {
            // 将HashMap转换为Vec以便序列化
            let env_vars: Vec<EnvVariableResponse> = config.env_vars
                .iter()
                .map(|(index, env)| EnvVariableResponse {
                    index: *index,
                    key: env.key.clone(),
                    value: env.value.clone(),
                })
                .collect();
            *state.shell_config.lock().unwrap() = Some(config);
            ApiResponse::success(env_vars)
        },
        Err(e) => ApiResponse::error(500, "Failed to load config file", &e.to_string()),
    }
}




///  更新环境变量
#[tauri::command]
pub fn update_env_variable(state: State<AppState>, env_var: EnvVariableRequest) -> ApiResponse<bool> {
    let mut config = state.shell_config.lock().unwrap();


    let env = EnvVariable {
        key: env_var.key,
        value: env_var.value,
    };
    
    match config.as_mut().unwrap().update_env(&env_var.index, &env) {
        Ok(_) => ApiResponse::success(true),
        Err(e) => ApiResponse::error(400, "Failed to update environment variable", &e.to_string()),
    }
}


/// 增加环境变量
#[tauri::command]
pub fn add_env_variable(state: State<AppState>, env_var: EnvVariableRequest) -> ApiResponse<bool> {
    let mut config = state.shell_config.lock().unwrap();


    let env = EnvVariable {
        key: env_var.key,
        value: env_var.value,
    };
    
    match config.as_mut().unwrap().add_env(&env) {
        Ok(_) => ApiResponse::success(true),
        Err(e) => ApiResponse::error(400, "Failed to update environment variable", &e.to_string()),
    }
}

/// 删除环境变量
#[tauri::command]
pub fn delete_env_variable(state: State<AppState>,  index: u64) -> ApiResponse<bool> {
    let mut config = state.shell_config.lock().unwrap();

    match config.as_mut().unwrap().remove_env(&index) {
        Ok(_) => ApiResponse::success(true),
        Err(e) => ApiResponse::error(400, "Failed to remove environment variable", &e.to_string()),
    }
}


/// 保存所有修改到文件
#[tauri::command]
pub fn save_config_file(state: State<AppState>) -> ApiResponse<bool> {
    let mut  config = state.shell_config.lock().unwrap();
   
    match config.as_mut().unwrap().save() {
        Ok(_) => ApiResponse::success(true),
        Err(e) => ApiResponse::error(500, "Failed to save config file", &e.to_string()),
    }
    
}
