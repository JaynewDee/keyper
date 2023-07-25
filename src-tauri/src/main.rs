#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod events;
use hookmap_core::event::Event;
use hookmap_core::{button::Button::*, event::ButtonEvent};
use tauri::{App, AppHandle, Manager};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    key: String,
}

fn main() -> Result<(), anyhow::Error> {
    let app_events_setup = |app: &mut App| {
        let handle = app.handle();

        std::thread::spawn(move || {
            let rx = hookmap_core::install_hook();

            while let Ok((event, native_handler)) = rx.recv() {
                match event {
                    Event::Button(event) => {
                        native_handler.dispatch();
                        match_event(event, &handle)
                    }
                    _ => continue,
                }
            }
        });
    };

    tauri::Builder::default()
        .setup(move |app| {
            app_events_setup(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

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

fn match_event(event: ButtonEvent, handle: &AppHandle) {
    match event.target {
        LeftButton => emit(&handle, "LeftClick"),
        RightButton => emit(&handle, "RightClick"),
        RightArrow => emit(&handle, "RightArrow"),
        UpArrow => emit(&handle, "UpArrow"),
        LeftArrow => emit(&handle, "LeftArrow"),
        DownArrow => emit(&handle, "DownArrow"),
        Space => emit(&handle, "Spacebar"),
        Enter => emit(&handle, "Enter"),
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
        A => emit(&handle, "A"),
        B => emit(&handle, "B"),
        C => emit(&handle, "C"),
        D => emit(&handle, "D"),
        E => emit(&handle, "E"),
        F => emit(&handle, "F"),
        G => emit(&handle, "G"),
        H => emit(&handle, "H"),
        I => emit(&handle, "I"),
        J => emit(&handle, "J"),
        K => emit(&handle, "K"),
        L => emit(&handle, "L"),
        M => emit(&handle, "M"),
        N => emit(&handle, "N"),
        O => emit(&handle, "O"),
        P => emit(&handle, "P"),
        Q => emit(&handle, "Q"),
        R => emit(&handle, "R"),
        S => emit(&handle, "S"),
        T => emit(&handle, "T"),
        U => emit(&handle, "U"),
        V => emit(&handle, "V"),
        W => emit(&handle, "W"),
        X => emit(&handle, "X"),
        Y => emit(&handle, "Y"),
        Z => emit(&handle, "Z"),
        _ => {}
    };
}
