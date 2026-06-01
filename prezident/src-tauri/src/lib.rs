use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
struct ProjectData {
    config: String,
    presentation: String,
    stylesheet: String,
}

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

#[tauri::command]
fn open_project(folder_path: String) -> Result<ProjectData, String> {
    let path = Path::new(&folder_path);

    let config = fs::read_to_string(path.join("config.json")).map_err(|e| e.to_string())?;
    let presentation = fs::read_to_string(path.join("presentation.md")).map_err(|e| e.to_string())?;
    let stylesheet = fs::read_to_string(path.join("style.css")).map_err(|e| e.to_string())?;

    Ok(ProjectData { config, presentation, stylesheet })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, save_project, open_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
