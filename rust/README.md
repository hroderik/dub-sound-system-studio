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

## Próximos passos

1. Implementar os demais kernels (reverb de placa/mola, phaser, flanger, sub, auto-wah).
2. Ligar os knobs do frontend aos comandos Tauri (soft takeover).
3. Adicionar separação de stems (htdemucs via ONNX Runtime).
4. Suporte a controlador MIDI físico (Web MIDI no frontend já funciona; nativo via
   midir/cpal).
5. Empacotar e publicar (inspirado no make-app.sh do DubStemMix).
