use std::fs;
use std::io::Write;
use std::sync::Mutex;
use zip::write::SimpleFileOptions;
use zip::ZipArchive;
use tauri::Manager;

struct TempFolder(Mutex<Option<String>>);

#[derive(serde::Serialize)]
struct OpenResult {
    config: String,
    presentation: String,
    stylesheet: String,
    temp_folder: String,
}

#[tauri::command]
fn open_codeprez(
    file_path: String,
    state: tauri::State<TempFolder>,
) -> Result<OpenResult, String> {
    if let Some(old) = state.0.lock().unwrap().take() {
        let _ = fs::remove_dir_all(&old);
    }

    let file = fs::File::open(&file_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let temp_dir = std::env::temp_dir().join(format!("codeprez_{}", ts));
    fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        let out_path = match entry.enclosed_name() {
            Some(p) => temp_dir.join(p),
            None => continue,
        };
        if entry.is_dir() {
            fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut out_file = fs::File::create(&out_path).map_err(|e| e.to_string())?;
            std::io::copy(&mut entry, &mut out_file).map_err(|e| e.to_string())?;
        }
    }

    let config = fs::read_to_string(temp_dir.join("config.json")).map_err(|e| e.to_string())?;
    let presentation = fs::read_to_string(temp_dir.join("presentation.md")).map_err(|e| e.to_string())?;
    let stylesheet = fs::read_to_string(temp_dir.join("style.css")).map_err(|e| e.to_string())?;
    let temp_folder = temp_dir.to_string_lossy().to_string();

    *state.0.lock().unwrap() = Some(temp_folder.clone());

    Ok(OpenResult { config, presentation, stylesheet, temp_folder })
}

#[tauri::command]
fn save_codeprez(
    file_path: String,
    config: String,
    presentation: String,
    stylesheet: String,
    assets_folder: String,
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

    if !assets_folder.is_empty() {
        if let Ok(entries) = fs::read_dir(&assets_folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let data = fs::read(&path).map_err(|e| e.to_string())?;
                    zip.start_file(format!("assets/{}", name), opts).map_err(|e| e.to_string())?;
                    zip.write_all(&data).map_err(|e| e.to_string())?;
                }
            }
        }
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(TempFolder(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![save_codeprez, open_codeprez])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let tauri::RunEvent::Exit = event {
                if let Some(folder) = app.state::<TempFolder>().0.lock().unwrap().take() {
                    let _ = fs::remove_dir_all(&folder);
                }
            }
        });
}
