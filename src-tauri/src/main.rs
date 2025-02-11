#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::header::CONTENT_TYPE;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use sysinfo::System;
use tauri::Manager;

use chksum_md5;

#[tauri::command]
async fn download_file(url: String, temp_path: String) -> Result<(), String> {
    let response = reqwest::get(&url).await.expect("Failed downloading file");
    let body = response.bytes().await.expect("Failed downloading file");
    let path = Path::new(&temp_path);

    let file = match File::create(&path) {
        Err(why) => Err(why.to_string()),
        Ok(file) => Ok(file),
    };
    file?
        .write_all(&body.to_vec())
        .expect(&"Failed writing file".to_string());
    Ok(())
}

#[tauri::command]
async fn unpack_archive(archive_path: String, extract_path: String) -> Result<(), String> {
    sevenz_rust::decompress_file(&archive_path, &extract_path).expect("complete");
    Ok(())
}

#[tauri::command]
async fn get_file_md5(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(file_path);
    if path.exists() {
        let digest = chksum_md5::chksum(path).expect("MD5 failed");
        Ok(digest.to_hex_lowercase())
    } else {
        Err("Path doesn't exist".to_string())
    }
}

#[tauri::command]
async fn check_processes(process: String) -> Result<(), String> {
    let s = System::new_all();
    let os_str = OsStr::new(&process);
    for _process in s.processes_by_exact_name(&os_str) {
        return Ok(());
    }
    return Err("No process with that name was found".to_string());
}

#[tauri::command]
async fn download_archive(url: String, temp_path: String, files: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header(CONTENT_TYPE, "application/json")
        .body(files)
        .send()
        .await
        .expect("Failed downloading file");
    let body = response.bytes().await.expect("Failed downloading file");
    let path = Path::new(&temp_path);

    let file = match File::create(&path) {
        Err(why) => Err(why.to_string()),
        Ok(file) => Ok(file),
    };
    file?
        .write_all(&body.to_vec())
        .expect(&"Failed writing file".to_string());
    Ok(())
}

fn main() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_process::init());
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }
    builder
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            download_file,
            unpack_archive,
            get_file_md5,
            check_processes,
            download_archive
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
