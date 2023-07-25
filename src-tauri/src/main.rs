#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod events;
use hookmap_core::button::Button::*;
use hookmap_core::event::Event;
use tauri::{AppHandle, Manager};
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    key: String,
}

fn main() -> Result<(), anyhow::Error> {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            fn emit(app_handle: &AppHandle, key: &str) {
                app_handle
                    .emit_to(
                        "main",
                        "key_event",
                        Payload {
                            key: key.to_string(),
                        },
                    )
                    .unwrap();
            }
            std::thread::spawn(move || {
                let rx = hookmap_core::install_hook();

                while let Ok((event, native_handler)) = rx.recv() {
                    match event {
                        Event::Button(event) => {
                            native_handler.dispatch();

                            match event.target {
                                LeftButton => emit(&handle, "LeftClick"),
                                RightButton => emit(&handle, "RightClick"),
                                RightArrow => emit(&handle, "RightArrow"),
                                UpArrow => emit(&handle, "UpArrow"),
                                LeftArrow => emit(&handle, "LeftArrow"),
                                DownArrow => emit(&handle, "DownArrow"),
                                Space => emit(&handle, "Spacebar"),
                                Backspace => emit(&handle, "Backspace"),
                                Tab => emit(&handle, "Tab"),
                                Key1 => emit(&handle, "1"),
                                Key2 => emit(&handle, "2"),
                                Key3 => emit(&handle, "3"),
                                Key4 => emit(&handle, "4"),
                                Key5 => emit(&handle, "5"),
                                Key6 => emit(&handle, "6"),
                                Key7 => emit(&handle, "7"),
                                Key8 => emit(&handle, "8"),
                                Key9 => emit(&handle, "9"),
                                Key0 => emit(&handle, "0"),

                                _ => {}
                            };
                        }
                        _ => continue,
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
