use std::collections::HashMap;

use log::info;
use tauri::{App, Manager};
use tauri::{CustomMenuItem, SystemTrayMenu};
use tauri::{SystemTray, SystemTrayEvent};
use tauri::{Window, Wry};
use tauri::api::notification::Notification;
use tauri::api::process::Command;

use crate::{indexing, utils, walk_exec};
use crate::file_view::{FileView, SearchResult};
use crate::idx_store::IDX_STORE;
use crate::user_setting::USER_SETTING;
use crate::walk_metrics::WalkMatrixView;

static mut WINDOW: Option<Window<Wry>> = None;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
async fn walk_metrics() -> WalkMatrixView {
    unsafe { walk_exec::get_walk_matrix() }
}

#[tauri::command]
async fn change_theme(theme: u8) {
    USER_SETTING.write().unwrap().set_theme(theme);
}

#[tauri::command]
async fn get_theme() -> u8 {
    USER_SETTING.read().unwrap().theme()
}

#[tauri::command]
async fn get_lang() -> String {
    USER_SETTING.write().unwrap().lang().to_string()
}

#[tauri::command]
async fn change_lang(lang: String) {
    USER_SETTING.write().unwrap().set_lang(lang);
}

#[tauri::command]
async fn add_exclude_path(path: String) -> u8 {
    if path.eq("/") {
        return 1;
    }
    if USER_SETTING
        .read()
        .unwrap()
        .exclude_index_path()
        .iter()
        .any(|x| x.eq(&path))
    {
        return 1;
    }

    match USER_SETTING.write().unwrap().add_exclude_index_path(path) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[tauri::command]
async fn get_exclude_paths() -> Vec<String> {
    USER_SETTING
        .read()
        .unwrap()
        .exclude_index_path()
        .iter()
        .map(|x| x.to_string())
        .collect()
}

#[tauri::command]
async fn remove_exclude_path(path: String) {
    USER_SETTING
        .write()
        .unwrap()
        .remove_exclude_index_path(path);
}

#[tauri::command]
async fn upgrade() {
    let _ = webbrowser::open("https://github.com/naaive/orange/releases");
}

#[tauri::command]
async fn suggest(kw: String) -> Vec<FileView> {
    info!("suggest kw :{}", kw);
    IDX_STORE.suggest(kw, 20)
}

#[tauri::command]
async fn search(mut kw: String, is_dir_opt: Option<bool>, ext_opt: Option<String>) -> SearchResult {
    info!("tauri::command-search kw :{}, ext_opt :{:?}", kw, ext_opt);
    if kw.eq("") {
        kw = "*".to_string();
    }
    IDX_STORE.search_with_filter(kw, 100, is_dir_opt, ext_opt)
}

#[tauri::command]
async fn open_file_in_terminal(kw: String) {
    info!("tauri::command-open_file_in_terminal: {}", kw);
    utils::open_file_path_in_terminal(kw.as_str());
}

#[tauri::command]
async fn open_file_in_explorer(kw: String) {
    info!("tauri::command-open_file_in_explorer: {}", kw);
    utils::open_file_path_in_explorer(kw.as_str());
}

#[tauri::command]
async fn open_file_path(kw: String) {
    utils::open_file_path(kw.as_str());
}

#[tauri::command]
async fn excel_automation_backend(file_path: String, template_path: String) {
    info!("excel_automation_backend_command: {}", file_path);
    utils::excel_automation_backend(file_path.as_str());
    let (mut rx, mut child) = Command::new_sidecar("excel")
        .expect("failed to create `excel` binary command")
        .spawn()
        .expect("Failed to spawn sidecar");
    let template_path = "/Users/kuanhsiaokuo/Developer/spare_projects/file_robots/src-tauri/excel_operators/basic/result_template.xlsx".to_string();
    // let excel_operator_cmd = format!("excel_operator {file_path} {template_path}");
    let excel_operator_cmd = "excel_operator";
    let execute_args = vec![file_path, template_path];
    // let mut execute_args = HashMap::new();
    // execute_args.insert("excel_file_path", file_path);
    // execute_args.insert("excel_template_path", template_path.to_string());
    // Command::envs(execute_args);
    info!("excel_operator_cmd: {}", excel_operator_cmd);
    let (mut rx, mut child) = Command::new_sidecar(excel_operator_cmd)
        .unwrap().args(execute_args)
        // .expect("failed to create `excel_operator` binary command")
        .spawn()
        .expect("Failed to spawn sidecar");
}

#[tauri::command]
async fn reindex() {
    indexing::reindex();
}

fn init_window(x: &mut App<Wry>) {
    let option = x.get_window("main");
    unsafe {
        WINDOW = option;
    }
}

fn handle_tray_event(event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "reindex" => {
                unsafe {
                    let _ = WINDOW.clone().unwrap().emit(
                        "reindex",
                        Payload {
                            message: "".to_string(),
                        },
                    );
                }
                indexing::reindex();
            }
            "upgrade" => {
                let _ = webbrowser::open("https://github.com/naaive/orange/releases");
            }
            _ => {}
        },
        _ => {}
    }
}

fn build_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let reindex = CustomMenuItem::new("reindex".to_string(), "Reindex");
    let upgrade = CustomMenuItem::new("upgrade".to_string(), "Upgrade");
    let tray_menu = SystemTrayMenu::new()
        .add_item(upgrade)
        // .add_item(reindex)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    tray
}

pub fn show() {
    let app = tauri::Builder::default()
        .setup(|app| {
            init_window(app);
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                // window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_file_path,
            open_file_in_terminal,
            open_file_in_explorer,
            excel_automation_backend,
            search,
            suggest,
            walk_metrics,
            change_theme,
            get_theme,
            upgrade,
            remove_exclude_path,
            add_exclude_path,
            change_lang,
            get_lang,
            reindex,
            get_exclude_paths
    ])
        .system_tray(build_tray())
        .on_system_tray_event(|_, event| handle_tray_event(event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
