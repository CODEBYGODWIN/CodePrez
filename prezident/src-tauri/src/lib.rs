use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipArchive;

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
    let presentation =
        fs::read_to_string(path.join("presentation.md")).map_err(|e| e.to_string())?;
    let stylesheet = fs::read_to_string(path.join("style.css")).map_err(|e| e.to_string())?;

    Ok(ProjectData {
        config,
        presentation,
        stylesheet,
    })
}

#[tauri::command]
fn save_codeprez(
    file_path: String,
    config: String,
    presentation: String,
    stylesheet: String,
) -> Result<(), String> {
    let file = fs::File::create(&file_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    zip.add_directory("assets/", opts).map_err(|e| e.to_string())?;
    zip.add_directory("env/", opts).map_err(|e| e.to_string())?;

    zip.start_file("config.json", opts).map_err(|e| e.to_string())?;
    zip.write_all(config.as_bytes()).map_err(|e| e.to_string())?;

    zip.start_file("presentation.md", opts).map_err(|e| e.to_string())?;
    zip.write_all(presentation.as_bytes()).map_err(|e| e.to_string())?;

    zip.start_file("style.css", opts).map_err(|e| e.to_string())?;
    zip.write_all(stylesheet.as_bytes()).map_err(|e| e.to_string())?;

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn open_codeprez(file_path: String) -> Result<ProjectData, String> {
    let file = fs::File::open(&file_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    let config = read_zip_text(&mut archive, "config.json")?;
    let presentation = read_zip_text(&mut archive, "presentation.md")?;
    let stylesheet = read_zip_text(&mut archive, "style.css")?;

    Ok(ProjectData {
        config,
        presentation,
        stylesheet,
    })
}

fn read_zip_text(archive: &mut ZipArchive<fs::File>, name: &str) -> Result<String, String> {
    let mut entry = archive
        .by_name(name)
        .map_err(|e| format!("{}: {}", name, e))?;
    let mut content = String::new();
    entry
        .read_to_string(&mut content)
        .map_err(|e| e.to_string())?;
    Ok(content)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            save_project,
            open_project,
            save_codeprez,
            open_codeprez
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
