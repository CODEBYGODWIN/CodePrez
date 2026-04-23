use std::fs;
use std::path::Path;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_project(
    folder_path: String,
    config: String,
    presentation: String,
    stylesheet: String,
) -> Result<(), String> {
    let path = Path::new(&folder_path);

    fs::create_dir_all(path.join("assets")).map_err(|e| e.to_string())?;
    fs::create_dir_all(path.join("env")).map_err(|e| e.to_string())?;

    fs::write(path.join("config.json"), &config).map_err(|e| e.to_string())?;
    fs::write(path.join("presentation.md"), &presentation).map_err(|e| e.to_string())?;
    fs::write(path.join("style.css"), &stylesheet).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, save_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
