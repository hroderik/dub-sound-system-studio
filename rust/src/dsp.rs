// dsp.rs — kernels de DSP em Rust (portáveis, sem alocação na thread de áudio).
//
// Equivalente ao tape_delay.c, mas em Rust. Compila para nativo (Linux/macOS/Windows)
// e, com target wasm32, para WASM (AudioWorklet).

const MAX_DELAY_SAMPLES: usize = 192000; // ~4s a 48kHz

pub struct TapeDelay {
    buffer: Vec<f32>,
    write_pos: usize,
    delay_samples: usize,
    feedback: f32,
    wet: f32,
    filter_state: f32,
    sample_rate: f32,
}

impl TapeDelay {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            buffer: vec![0.0; MAX_DELAY_SAMPLES],
            write_pos: 0,
            delay_samples: (0.375 * sample_rate) as usize,
            feedback: 0.4,
            wet: 0.0,
            filter_state: 0.0,
            sample_rate,
        }
    }

    pub fn set_params(&mut self, time_sec: f32, feedback: f32, wet: f32) {
        let mut ds = (time_sec * self.sample_rate) as usize;
        if ds < 1 { ds = 1; }
        if ds >= MAX_DELAY_SAMPLES { ds = MAX_DELAY_SAMPLES - 1; }
        self.delay_samples = ds;
        self.feedback = feedback.clamp(0.0, 0.95);
        self.wet = wet.clamp(0.0, 1.0);
    }

    /// Processa um bloco. Retorna (dry, wet) mixados em out.
    pub fn process(&mut self, input: &[f32], out: &mut [f32]) {
        for i in 0..input.len() {
            let mut read_pos = self.write_pos as isize - self.delay_samples as isize;
            if read_pos < 0 { read_pos += MAX_DELAY_SAMPLES as isize; }
            let delayed = self.buffer[read_pos as usize];

            // filtro passa-baixa (eco escuro de fita)
            self.filter_state += 0.15 * (delayed - self.filter_state);
            delayed = self.filter_state;

            self.buffer[self.write_pos] = input[i] + delayed * self.feedback;
            self.write_pos += 1;
            if self.write_pos >= MAX_DELAY_SAMPLES { self.write_pos = 0; }

            out[i] = input[i] + delayed * self.wet;
        }
    }
}

/// Função pura (sem estado persistente) para uso one-shot pelo comando Tauri.
/// O host mantém o estado do TapeDelay entre chamadas.
pub fn tape_delay_process_block(input: &[f32], time: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    let mut d = TapeDelay::new(sr);
    d.set_params(time, fb, wet);
    let mut out = vec![0.0f32; input.len()];
    d.process(input, &mut out);
    out
}
