use std::fs;
use std::path::Path;
use std::path::PathBuf;

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

#[tauri::command]
fn list_files(folder_path: String) -> Result<Vec<String>, String> {
  let entries = std::fs::read_dir(folder_path)
    .map_err(|e| e.to_string())?;

  let mut files = Vec::new();

  for entry in entries {
    let entry = entry.map_err(|e| e.to_string())?;
    let path = entry.path();

    if path.is_file() {
      if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        files.push(name.to_string());
      }
    }
  }

  Ok(files)
}

#[tauri::command]
fn copy_image_to_assets(source_path: String, folder_path: String) -> Result<(), String> {
  let source = PathBuf::from(&source_path);
  let file_name = source
    .file_name()
    .ok_or("Nom de fichier invalide")?
    .to_owned();

  let destination = PathBuf::from(folder_path).join(file_name);

  std::fs::copy(&source, &destination)
    .map(|_| ())
    .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, save_project, open_project, list_files, copy_image_to_assets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
