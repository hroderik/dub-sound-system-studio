// main.rs — ponto de entrada do executável nativo (Tauri 2).
//
// A interface é o index.html (reaproveitado). O DSP pesado roda em Rust
// (src/dsp.rs) e é exposto ao frontend via comandos Tauri.
//
// Para rodar em dev:  cargo tauri dev
// Para compilar:      cargo tauri build

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dsp;

use tauri::Manager;

#[tauri::command]
fn process_tape_delay(input: Vec<f32>, time: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    // Wrapper fino: o DSP real vive em dsp.rs (sem alocação na thread de áudio).
    dsp::tape_delay_process_block(&input, time, fb, wet, sr)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_tape_delay])
        .setup(|app| {
            // Janela principal: carrega o index.html empacotado
            let _ = app;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao rodar dub-mixer");
}
