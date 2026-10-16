# 📝 Notas — Como instalar o Rust e compilar a versão executável

> Arquivo de notas. Pode ser apagado quando a versão executável estiver pronta.

## 1. Instalar o Rust

### Linux / macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# siga as instruções na tela; depois:
source $HOME/.cargo/env
rustc --version
cargo --version
```

### Windows
Baixe e rode o instalador: https://rustup.rs/ (rustup-init.exe).
Marque "Add to PATH" quando perguntado. Depois abra um terminal novo:
```powershell
rustc --version
cargo --version
```

## 2. Instalar o Tauri CLI

```bash
cargo install tauri-cli
# ou, alternativamente:
npm install -g @tauri-apps/cli
```

## 3. Instalar dependências do sistema (Linux)

```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libxdo-devel libappindicator-gtk3-devel librsvg2-devel
```

macOS e Windows não precisam de dependências extras.

## 4. Rodar em modo desenvolvimento

```bash
cd rust
cargo tauri dev
```
Abre a janela com o `index.html` empacotado.

## 5. Compilar o executável

```bash
cd rust
cargo tauri build
```

Saída:
- **Windows:** `rust/target/release/bundle/msi/*.msi` e `*.exe`
- **macOS:** `rust/target/release/bundle/dmg/*.dmg` e `.app`
- **Linux:** `rust/target/release/bundle/appimage/*.AppImage` e `.deb`

## 6. O que falta implementar (após compilar)

- [ ] Ligar os knobs do frontend aos comandos Tauri (soft takeover).
- [ ] Adicionar separação de stems (htdemucs via ONNX Runtime).
- [ ] Suporte a controlador MIDI físico (midir/cpal).
- [ ] Empacotar e publicar (inspirado no `make-app.sh` do DubStemMix).
- [ ] Compilar `dsp/tape_delay.c` com emscripten → WASM para a rota web.

## 7. Compilar o kernel C → WASM (opcional, para a rota web)

```bash
# Instalar emscripten
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk && ./emsdk install latest && ./emsdk activate latest
source ./emsdk_env.sh

# Compilar
cd rust/../dsp
emcc tape_delay.c -O3 -s WASM=1 \
  -s EXPORTED_FUNCTIONS="['_tape_delay_init','_tape_delay_set_params','_tape_delay_process']" \
  -o tape_delay.wasm
```

## Referências

- Rust: https://www.rust-lang.org/
- Tauri 2: https://tauri.app/
- Emscripten: https://emscripten.org/
- DubStemMix (referência): https://github.com/hroderik/DubStemMix
- ROADMAP-REA: docs/ROADMAP-REA.md
