use tauri::{AppHandle, Event};

use crate::ProgressPayload;

// pub fn quit_event() {
//     std::process::exit(0);
// }

pub fn progress_event(handle: AppHandle, event: Event) {
    if let Some(payload) = event.payload() {
        let data: Result<ProgressPayload, _> = serde_json::from_str(payload);
    
        match data {
            Ok(data) => {
                match data.progress {
                    10 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/progress/progress-10.png").to_vec())).unwrap(),
                    20 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/progress/progress-20.png").to_vec())).unwrap(),
                    30 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/progress/progress-30.png").to_vec())).unwrap(),
                    40 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/progress/progress-40.png").to_vec())).unwrap(),
                    50 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/progress/progress-50.png").to_vec())).unwrap(),
                    60 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/progress/progress-60.png").to_vec())).unwrap(),
                    70 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/progress/progress-70.png").to_vec())).unwrap(),
                    80 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/progress/progress-80.png").to_vec())).unwrap(),
                    90 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/progress/progress-90.png").to_vec())).unwrap(),
                    100 => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/icon.png").to_vec())).unwrap(),
                    _ => handle.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../../icons/icon.png").to_vec())).unwrap(),
                }
            }
            Err(e) => {
                println!("Failed to deserialize payload: {}", e);
            }
        }
    }
}