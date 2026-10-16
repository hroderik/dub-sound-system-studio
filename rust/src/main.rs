// main.rs — ponto de entrada do executável nativo (Tauri 2).
//
// A interface é o index.html (reaproveitado). O DSP pesado roda em Rust
// (src/dsp.rs) e é exposto ao frontend via comandos Tauri.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dsp;

use tauri::Manager;

// --- Comandos Tauri (um por kernel) ---
#[tauri::command]
fn process_tape_delay(input: Vec<f32>, time: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    dsp::tape_delay_process_block(&input, time, fb, wet, sr)
}
#[tauri::command]
fn process_spring_reverb(input: Vec<f32>, wet: f32, sr: f32) -> Vec<f32> {
    dsp::spring_reverb_process_block(&input, wet, sr)
}
#[tauri::command]
fn process_phaser(input: Vec<f32>, rate: f32, depth: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    dsp::phaser_process_block(&input, rate, depth, fb, wet, sr)
}
#[tauri::command]
fn process_flanger(input: Vec<f32>, rate: f32, depth: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    dsp::flanger_process_block(&input, rate, depth, fb, wet, sr)
}
#[tauri::command]
fn process_sub(input: Vec<f32>, cutoff: f32, gain: f32, sr: f32) -> Vec<f32> {
    dsp::sub_process_block(&input, cutoff, gain, sr)
}
#[tauri::command]
fn process_autowah(input: Vec<f32>, base_freq: f32, range: f32, q: f32, wet: f32, sr: f32) -> Vec<f32> {
    dsp::autowah_process_block(&input, base_freq, range, q, wet, sr)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            process_tape_delay, process_spring_reverb, process_phaser,
            process_flanger, process_sub, process_autowah
        ])
        .setup(|app| {
            let _ = app;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro ao rodar dub-mixer");
}
