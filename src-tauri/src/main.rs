#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod events;
use hookmap_core::button::Button;
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
                                Button::RightArrow => emit(&handle, "RightArrow"),
                                Button::UpArrow => emit(&handle, "UpArrow"),
                                Button::LeftArrow => emit(&handle, "LeftArrow"),
                                Button::DownArrow => emit(&handle, "DownArrow"),
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
