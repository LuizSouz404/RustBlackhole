#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{SystemTray, Manager, SystemTrayEvent, SystemTrayMenu, CustomMenuItem};
use tauri_plugin_positioner::{Position, WindowExt};

mod events;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct ProgressPayload {
    progress: i32,
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");

    let system_tray_menu = SystemTrayMenu::new()
        .add_item(quit);

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu).with_id("Blackhole"))
        .setup(|app| {
            let handle = app.handle();

            app.listen_global("quit", | _ | {
                std::process::exit(0)
            });

            app.listen_global("progress", move | event | {
               events::progress_event(handle.clone(), event)
            });
        
            Ok(())
        })
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    // use TrayCenter as initial window position
                    let _ = window.move_window(Position::TrayCenter);
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                },
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
