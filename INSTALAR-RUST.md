# 📝 Notas — Como instalar o Rust e compilar a versão executável

> Arquivo de notas. Pode ser apagado quando a versão executável estiver pronta.

## 1. Instalar o Rust

### Linux / macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
cargo --version
```

### Windows
Baixe e rode o instalador: https://rustup.rs/ (rustup-init.exe).
Marque "Add to PATH" quando perguntado.

## 2. Instalar o Tauri CLI

```bash
cargo install tauri-cli
```

## 3. Dependências do sistema (Linux)

```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

## 4. Rodar em dev

```bash
cd rust
cargo tauri dev
```

## 5. Compilar o executável

```bash
cd rust
cargo tauri build
```

Saída:
- **Windows:** `rust/target/release/bundle/msi/*.msi`
- **macOS:** `rust/target/release/bundle/dmg/*.dmg`
- **Linux:** `rust/target/release/bundle/appimage/*.AppImage`

## 6. O que falta implementar

- [ ] Ligar os knobs do frontend aos comandos Tauri (soft takeover).
- [ ] Adicionar separação de stems (htdemucs via ONNX Runtime).
- [ ] Suporte a controlador MIDI físico (midir/cpal).
- [ ] Empacotar e publicar.

## 7. Compilar o kernel C → WASM (opcional)

```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk && ./emsdk install latest && ./emsdk activate latest
source ./emsdk_env.sh
cd ../dsp
emcc tape_delay.c -O3 -s WASM=1 -s EXPORTED_FUNCTIONS="['_tape_delay_init','_tape_delay_set_params','_tape_delay_process']" -o tape_delay.wasm
```

## Referências

- Rust: https://www.rust-lang.org/
- Tauri 2: https://tauri.app/
- Emscripten: https://emscripten.org/
- DubStemMix: https://github.com/hroderik/DubStemMix
- ROADMAP-REA: docs/ROADMAP-REA.md
