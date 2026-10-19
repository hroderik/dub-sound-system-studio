// main.rs — ponto de entrada do executável nativo (Tauri 2).
//
// A interface é o index.html (reaproveitado). O DSP pesado roda em Rust
// (src/dsp.rs) e é exposto ao frontend via comandos Tauri.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dsp;

// --- Limites de segurança ---
// Evita que um frontend com bug (ou hostil) mande buffers gigantes ou valores
// absurdos (NaN/Inf) e derrube o processo por memória ou corrompa o DSP.
const MAX_SAMPLES: usize = 8_000_000; // ~3 min de áudio estéreo a 44,1 kHz
const SR_LO: f32 = 8_000.0;
const SR_HI: f32 = 192_000.0;

fn san(v: f32, lo: f32, hi: f32) -> f32 {
    if v.is_finite() { v.clamp(lo, hi) } else { lo }
}
fn bloc(v: &[f32]) -> &[f32] {
    &v[..v.len().min(MAX_SAMPLES)]
}

// --- Comandos Tauri (um por kernel) ---
#[tauri::command]
fn process_tape_delay(input: Vec<f32>, time: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    dsp::tape_delay_process_block(bloc(&input), san(time, 0.01, 4.0), san(fb, 0.0, 0.99),
                                  san(wet, 0.0, 1.0), san(sr, SR_LO, SR_HI))
}
#[tauri::command]
fn process_spring_reverb(input: Vec<f32>, wet: f32, sr: f32) -> Vec<f32> {
    dsp::spring_reverb_process_block(bloc(&input), san(wet, 0.0, 1.0), san(sr, SR_LO, SR_HI))
}
#[tauri::command]
fn process_phaser(input: Vec<f32>, rate: f32, depth: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    dsp::phaser_process_block(bloc(&input), san(rate, 0.01, 40.0), san(depth, 0.0, 1.0),
                              san(fb, -0.99, 0.99), san(wet, 0.0, 1.0), san(sr, SR_LO, SR_HI))
}
#[tauri::command]
fn process_flanger(input: Vec<f32>, rate: f32, depth: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    dsp::flanger_process_block(bloc(&input), san(rate, 0.01, 40.0), san(depth, 0.0, 1.0),
                               san(fb, -0.99, 0.99), san(wet, 0.0, 1.0), san(sr, SR_LO, SR_HI))
}
#[tauri::command]
fn process_sub(input: Vec<f32>, cutoff: f32, gain: f32, sr: f32) -> Vec<f32> {
    dsp::sub_process_block(bloc(&input), san(cutoff, 20.0, 500.0), san(gain, 0.0, 4.0),
                           san(sr, SR_LO, SR_HI))
}
#[tauri::command]
fn process_autowah(input: Vec<f32>, base_freq: f32, range: f32, q: f32, wet: f32, sr: f32) -> Vec<f32> {
    dsp::autowah_process_block(bloc(&input), san(base_freq, 40.0, 5_000.0), san(range, 0.0, 4.0),
                               san(q, 0.1, 30.0), san(wet, 0.0, 1.0), san(sr, SR_LO, SR_HI))
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
