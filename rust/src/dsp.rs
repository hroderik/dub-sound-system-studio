// dsp.rs — kernels de DSP em Rust (portáveis, sem alocação na thread de áudio).
//
// Equivalente aos kernels do DubDSP (C), portados para Rust. Compila para nativo
// (Linux/macOS/Windows) e, com target wasm32, para WASM (AudioWorklet).

const MAX_DELAY_SAMPLES: usize = 192000; // ~4s a 48kHz

// ============================================================
// 1. TAPE DELAY (delay de fita com feedback e filtro passa-baixa)
// ============================================================
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
    pub fn process(&mut self, input: &[f32], out: &mut [f32]) {
        for i in 0..input.len() {
            let mut read_pos = self.write_pos as isize - self.delay_samples as isize;
            if read_pos < 0 { read_pos += MAX_DELAY_SAMPLES as isize; }
            let delayed = self.buffer[read_pos as usize];
            self.filter_state += 0.15 * (delayed - self.filter_state);
            let filtered = self.filter_state;
            self.buffer[self.write_pos] = input[i] + filtered * self.feedback;
            self.write_pos += 1;
            if self.write_pos >= MAX_DELAY_SAMPLES { self.write_pos = 0; }
            out[i] = input[i] + filtered * self.wet;
        }
    }
}

pub fn tape_delay_process_block(input: &[f32], time: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    let mut d = TapeDelay::new(sr);
    d.set_params(time, fb, wet);
    let mut out = vec![0.0f32; input.len()];
    d.process(input, &mut out);
    out
}

// ============================================================
// 2. SPRING REVERB (reverb de mola — decaimento + resonancias metalicas)
// ============================================================
pub struct SpringReverb {
    // Comb filters (4 linhas de atraso)
    comb_buf: [Vec<f32>; 4],
    comb_pos: [usize; 4],
    comb_times: [usize; 4],
    comb_fb: f32,
    // Allpass (2)
    ap_buf: [Vec<f32>; 2],
    ap_pos: [usize; 2],
    ap_times: [usize; 2],
    wet: f32,
    sample_rate: f32,
}

impl SpringReverb {
    pub fn new(sample_rate: f32) -> Self {
        // tempos de comb em ms (primos relativos para evitar modos)
        let ct = [29.7, 37.1, 41.1, 43.7];
        let at = [5.0, 1.7];
        let mut comb_buf = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut comb_times = [0usize; 4];
        for i in 0..4 {
            comb_times[i] = (ct[i] * 0.001 * sample_rate) as usize;
            comb_buf[i] = vec![0.0; comb_times[i].max(1)];
        }
        let mut ap_buf = [Vec::new(), Vec::new()];
        let mut ap_times = [0usize; 2];
        for i in 0..2 {
            ap_times[i] = (at[i] * 0.001 * sample_rate) as usize;
            ap_buf[i] = vec![0.0; ap_times[i].max(1)];
        }
        Self {
            comb_buf, comb_pos: [0; 4], comb_times, comb_fb: 0.78,
            ap_buf, ap_pos: [0; 2], ap_times,
            wet: 0.0, sample_rate,
        }
    }
    pub fn set_params(&mut self, wet: f32) {
        self.wet = wet.clamp(0.0, 1.0);
    }
    pub fn process(&mut self, input: &[f32], out: &mut [f32]) {
        for i in 0..input.len() {
            let mut rev = 0.0;
            for c in 0..4 {
                let idx = self.comb_pos[c];
                let delayed = self.comb_buf[c][idx];
                self.comb_buf[c][idx] = input[i] + delayed * self.comb_fb;
                self.comb_pos[c] = (idx + 1) % self.comb_buf[c].len();
                rev += delayed;
            }
            rev *= 0.25;
            // allpass
            for a in 0..2 {
                let idx = self.ap_pos[a];
                let delayed = self.ap_buf[a][idx];
                let x = rev;
                rev = delayed + 0.5 * x; // simplificado
                self.ap_buf[a][idx] = x - 0.5 * delayed;
                self.ap_pos[a] = (idx + 1) % self.ap_buf[a].len();
            }
            out[i] = input[i] + rev * self.wet;
        }
    }
}

pub fn spring_reverb_process_block(input: &[f32], wet: f32, sr: f32) -> Vec<f32> {
    let mut r = SpringReverb::new(sr);
    r.set_params(wet);
    let mut out = vec![0.0f32; input.len()];
    r.process(input, &mut out);
    out
}

// ============================================================
// 3. PHASER (allpass em cascata + LFO)
// ============================================================
pub struct Phaser {
    ap: [f32; 4], // estados dos allpass
    lfo_phase: f32,
    rate: f32,
    depth: f32,
    feedback: f32,
    wet: f32,
    sample_rate: f32,
    fb_state: f32,
}

impl Phaser {
    pub fn new(sample_rate: f32) -> Self {
        Self { ap: [0.0; 4], lfo_phase: 0.0, rate: 0.5, depth: 0.7, feedback: 0.5, wet: 0.0, sample_rate, fb_state: 0.0 }
    }
    pub fn set_params(&mut self, rate: f32, depth: f32, feedback: f32, wet: f32) {
        self.rate = rate.clamp(0.01, 10.0);
        self.depth = depth.clamp(0.0, 1.0);
        self.feedback = feedback.clamp(0.0, 0.95);
        self.wet = wet.clamp(0.0, 1.0);
    }
    pub fn process(&mut self, input: &[f32], out: &mut [f32]) {
        let lfo_inc = 2.0 * std::f32::consts::PI * self.rate / self.sample_rate;
        for i in 0..input.len() {
            self.lfo_phase += lfo_inc;
            if self.lfo_phase > 2.0 * std::f32::consts::PI { self.lfo_phase -= 2.0 * std::f32::consts::PI; }
            // coeficiente allpass modulado
            let coef = 0.1 + 0.8 * self.depth * (0.5 + 0.5 * self.lfo_phase.sin());
            let x = input[i] + self.fb_state * self.feedback;
            // cascata de 4 allpass
            let mut y = x;
            for a in 0..4 {
                let prev = self.ap[a];
                y = coef * (y - prev) + prev; // allpass simplificado
                self.ap[a] = y;
            }
            self.fb_state = y;
            out[i] = input[i] * (1.0 - self.wet) + y * self.wet;
        }
    }
}

pub fn phaser_process_block(input: &[f32], rate: f32, depth: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    let mut p = Phaser::new(sr);
    p.set_params(rate, depth, fb, wet);
    let mut out = vec![0.0f32; input.len()];
    p.process(input, &mut out);
    out
}

// ============================================================
// 4. FLANGER (delay curto modulado + feedback)
// ============================================================
pub struct Flanger {
    buffer: Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
    rate: f32,
    depth: f32,
    feedback: f32,
    wet: f32,
    sample_rate: f32,
    max_delay: usize,
}

impl Flanger {
    pub fn new(sample_rate: f32) -> Self {
        let max_delay = (0.02 * sample_rate) as usize; // 20ms max
        Self {
            buffer: vec![0.0; max_delay + 1],
            write_pos: 0,
            lfo_phase: 0.0,
            rate: 0.25,
            depth: 0.5,
            feedback: 0.5,
            wet: 0.0,
            sample_rate,
            max_delay,
        }
    }
    pub fn set_params(&mut self, rate: f32, depth: f32, fb: f32, wet: f32) {
        self.rate = rate.clamp(0.01, 10.0);
        self.depth = depth.clamp(0.0, 1.0);
        self.feedback = fb.clamp(0.0, 0.95);
        self.wet = wet.clamp(0.0, 1.0);
    }
    pub fn process(&mut self, input: &[f32], out: &mut [f32]) {
        let lfo_inc = 2.0 * std::f32::consts::PI * self.rate / self.sample_rate;
        let base_delay = (0.005 * self.sample_rate) as usize; // 5ms base
        for i in 0..input.len() {
            self.lfo_phase += lfo_inc;
            if self.lfo_phase > 2.0 * std::f32::consts::PI { self.lfo_phase -= 2.0 * std::f32::consts::PI; }
            let modulation = (0.5 + 0.5 * self.lfo_phase.sin()) * self.depth;
            let delay = base_delay + (modulation * (self.max_delay - base_delay) as f32) as usize;
            let mut read_pos = self.write_pos as isize - delay as isize;
            if read_pos < 0 { read_pos += (self.max_delay + 1) as isize; }
            let delayed = self.buffer[read_pos as usize];
            self.buffer[self.write_pos] = input[i] + delayed * self.feedback;
            self.write_pos = (self.write_pos + 1) % (self.max_delay + 1);
            out[i] = input[i] * (1.0 - self.wet) + delayed * self.wet;
        }
    }
}

pub fn flanger_process_block(input: &[f32], rate: f32, depth: f32, fb: f32, wet: f32, sr: f32) -> Vec<f32> {
    let mut f = Flanger::new(sr);
    f.set_params(rate, depth, fb, wet);
    let mut out = vec![0.0f32; input.len()];
    f.process(input, &mut out);
    out
}

// ============================================================
// 5. SUB (reforço de sub-grave: lowpass + ganho)
// ============================================================
pub struct SubBoost {
    lp_state: f32,
    cutoff: f32,
    gain: f32,
    sample_rate: f32,
}

impl SubBoost {
    pub fn new(sample_rate: f32) -> Self {
        Self { lp_state: 0.0, cutoff: 60.0, gain: 1.0, sample_rate }
    }
    pub fn set_params(&mut self, cutoff: f32, gain: f32) {
        self.cutoff = cutoff.clamp(20.0, 200.0);
        self.gain = gain.clamp(0.0, 4.0);
    }
    pub fn process(&mut self, input: &[f32], out: &mut [f32]) {
        let rc = 1.0 / (2.0 * std::f32::consts::PI * self.cutoff);
        let dt = 1.0 / self.sample_rate;
        let alpha = dt / (rc + dt);
        for i in 0..input.len() {
            self.lp_state += alpha * (input[i] - self.lp_state);
            out[i] = input[i] + self.lp_state * (self.gain - 1.0);
        }
    }
}

pub fn sub_process_block(input: &[f32], cutoff: f32, gain: f32, sr: f32) -> Vec<f32> {
    let mut s = SubBoost::new(sr);
    s.set_params(cutoff, gain);
    let mut out = vec![0.0f32; input.len()];
    s.process(input, &mut out);
    out
}

// ============================================================
// 6. AUTO-WAH (filtro passa-banda com frequência modulada por envelope)
// ============================================================
pub struct AutoWah {
    bp_state1: f32,
    bp_state2: f32,
    env: f32,
    base_freq: f32,
    range: f32,
    q: f32,
    wet: f32,
    sample_rate: f32,
}

impl AutoWah {
    pub fn new(sample_rate: f32) -> Self {
        Self { bp_state1: 0.0, bp_state2: 0.0, env: 0.0, base_freq: 500.0, range: 2000.0, q: 5.0, wet: 0.0, sample_rate }
    }
    pub fn set_params(&mut self, base_freq: f32, range: f32, q: f32, wet: f32) {
        self.base_freq = base_freq.clamp(200.0, 2000.0);
        self.range = range.clamp(0.0, 4000.0);
        self.q = q.clamp(1.0, 20.0);
        self.wet = wet.clamp(0.0, 1.0);
    }
    pub fn process(&mut self, input: &[f32], out: &mut [f32]) {
        let env_attack = 0.01;
        let env_release = 0.001;
        for i in 0..input.len() {
            let level = input[i].abs();
            if level > self.env {
                self.env += env_attack * (level - self.env);
            } else {
                self.env += env_release * (level - self.env);
            }
            let freq = self.base_freq + self.env * self.range;
            // coeficientes do filtro passa-banda (biquad simplificado)
            let w0 = 2.0 * std::f32::consts::PI * freq / self.sample_rate;
            let alpha = w0 / (2.0 * self.q);
            // processamento simplificado: passa-banda via estado
            let bp = (1.0 - alpha) * self.bp_state1 + alpha * (input[i] - self.bp_state2);
            self.bp_state2 = self.bp_state1;
            self.bp_state1 = bp;
            out[i] = input[i] * (1.0 - self.wet) + bp * self.wet;
        }
    }
}

pub fn autowah_process_block(input: &[f32], base_freq: f32, range: f32, q: f32, wet: f32, sr: f32) -> Vec<f32> {
    let mut a = AutoWah::new(sr);
    a.set_params(base_freq, range, q, wet);
    let mut out = vec![0.0f32; input.len()];
    a.process(input, &mut out);
    out
}
