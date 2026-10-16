# Versão executável em Rust (Tauri 2)

> **Status:** scaffold. Nada compilado ainda — requer Rust + Node no seu PC.

## Pré-requisitos

- Rust: https://rustup.rs/  (`rustup default stable`)
- Node.js 22+ e npm
- Tauri CLI: `cargo install tauri-cli` (ou `npm i -g @tauri-apps/cli`)

## Como rodar em dev

```bash
cd rust
cargo tauri dev
```

A janela abre com o `index.html` empacotado (servido de `../`).

## Como compilar

```bash
cd rust
cargo tauri build
```

Gera `.exe`/`.msi` (Windows), `.app`/`.dmg` (macOS), `.AppImage`/`.deb` (Linux).

## Estrutura

```
rust/
├── Cargo.toml          # dependências (tauri 2, serde)
├── tauri.conf.json     # config da janela e do bundle
├── src/
│   ├── main.rs         # ponto de entrada + comando process_tape_delay
│   └── dsp.rs          # kernels de DSP em Rust (TapeDelay, ...)
└── (futuro) src/dsp/   # outros kernels (reverb, phaser, flanger, sub, auto-wah)
```

## Relação com o web app

- A interface é o `index.html` (reaproveitado 100%).
- O DSP pesado vive em Rust (`src/dsp.rs`) e é exposto ao frontend via
  comandos Tauri (`process_tape_delay`).
- O kernel C (`dsp/tape_delay.c`) é o mesmo algoritmo, portável para WASM.

## Status

- [x] 6 kernels DSP implementados em Rust: TapeDelay, SpringReverb, Phaser, Flanger, SubBoost, AutoWah.
- [x] Comandos Tauri expostos: process_tape_delay, process_spring_reverb, process_phaser, process_flanger, process_sub, process_autowah.
- [ ] Ligar os knobs do frontend aos comandos Tauri (soft takeover).
- [ ] Adicionar separação de stems (htdemucs via ONNX Runtime).
- [ ] Suporte a controlador MIDI físico (midir/cpal).
- [ ] Empacotar e publicar (inspirado no make-app.sh do DubStemMix).
