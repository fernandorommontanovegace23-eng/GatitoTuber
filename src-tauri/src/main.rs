#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{thread, time::Duration};
use tauri::Emitter;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let h = app.handle().clone();
            // Lee W A S D y espacio 30 veces por segundo y avisa solo si algo cambia.
            thread::spawn(move || {
                let mut last: u8 = 255;
                loop {
                    let mut m: u8 = 0;
                    #[cfg(windows)]
                    unsafe {
                        use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
                        for (i, k) in [0x57, 0x41, 0x53, 0x44, 0x20].iter().enumerate() {
                            if (GetAsyncKeyState(*k) as u16) & 0x8000 != 0 {
                                m |= 1 << i;
                            }
                        }
                    }
                    if m != last {
                        last = m;
                        let _ = h.emit("keys", m);
                    }
                    thread::sleep(Duration::from_millis(33));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error al iniciar GatitoTuber");
}
