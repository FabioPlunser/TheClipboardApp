use enigo::{Enigo, Mouse, Settings};
use serde::Serialize;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Manager;
use window_vibrancy::*;

#[derive(Debug, Clone, Serialize)]
struct Position {
    x: i32,
    y: i32,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn show_clipboard(app: AppHandle) {
    println!("Ctrl+Shift+C Pressed!");
    let window = app.get_webview_window("main").unwrap();
    let enigo = Enigo::new(&Settings::default()).unwrap();
    let mouse_location = enigo.location();
    match mouse_location {
        Ok((x, y)) => {
            app.emit("show_clipboard", Position { x, y }).unwrap();
            println!("{:?}", mouse_location);
        }
        Err(e) => {
            println!("Mouse location not found: {:?}", e);
        }
    }
    window.show().unwrap();
    window.set_focus().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let shortcut =
                    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyC);

                let plugin = tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app: &AppHandle, current_shortcut, event| {
                        if current_shortcut == &shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    show_clipboard(app.clone());
                                }
                                ShortcutState::Released => {
                                    println!("Ctrl+Shift+C Released!");
                                }
                            }
                        }
                    })
                    .build();

                app.handle().plugin(plugin)?;
                app.handle().plugin(tauri_plugin_positioner::init())?;

                app.global_shortcut().register(shortcut.clone()).unwrap();

                let window = app.get_webview_window("main").unwrap();

                #[cfg(target_os = "macos")]
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

                #[cfg(target_os = "windows")]
                apply_acrylic(&window, Some((18, 18, 18, 125)))
                    .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

                window.hide().unwrap();

                // window.clone().on_window_event(move |event| {
                //     if let tauri::WindowEvent::Focused(is_focused) = event {
                //         if !is_focused {
                //             window.hide().unwrap();
                //         }
                //     }
                // });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
