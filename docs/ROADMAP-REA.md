# 🗺️ ROADMAP — Versão executável via engenharia reversa (REA)

> **Status:** planejamento. Nada implementado ainda.
> **Objetivo:** transformar esta mesa web em um **executável otimizado** que rode
> nativo em **Linux, macOS e Windows**, com as mesmas funções — mais a separação de
> stems e suporte a controlador MIDI físico.

---

## 1. Por que engenharia reversa, e não reescrever do zero?

O app web já funciona, mas tem três tetos que a web não resolve:

| Limite da versão web | Por quê |
|---|---|
| O áudio do YouTube **não entra** no grafo Web Audio | O `<iframe>` é uma caixa-preta de outra origem. Dá para controlar o volume, não processar o som. |
| **Sem separação de stems** | Não dá para "tirar o vocal" de um MP3 só com Web Audio. Isso exige inferência de rede neural (ONNX). |
| **Sem acesso a MIDI/ASIO de verdade** | Latência e controle de hardware ficam limitados. |

Já existe um projeto que resolveu exatamente esses três problemas, com a mesma alma
dub: o **[DubStemMix](https://github.com/hroderik/DubStemMix)** — console nativo
(macOS 15+, Apple Silicon, Swift + C, MIT).

Em vez de reescrever isso do zero (e redescobrir todos os problemas de tempo real),
a ideia é usar **engenharia reversa assistida por agentes** para **entender como ele
resolve** esses pontos e **portar** as decisões para uma versão multiplataforma.

### O que o DubStemMix tem que interessa

Do README e da organização do código do projeto:

- **`Sources/DubDSP`** — os *kernels* de DSP em **C**: *tape delay*, reverb de placa
  e de mola, phaser, flanger, cadeia de master, sub, auto-wah. Com uma regra de ouro
  explícita: **sem alocação e sem lock na thread de áudio**.
- **`Sources/DubStemMixCore`** — o motor de áudio (`AVAudioEngine`), o **roteamento
  de buses**, o controlador MIDI, a lógica de mix (`MixController`), projetos,
  setlists, hospedagem de plugins, gravação, detecção de tempo.
- **`Sources/StemSplit`** — separação de stems: decodificação, *overlap-add*,
  inferência **htdemucs_ft via ONNX Runtime**, download e verificação do modelo.
- **`Sources/DubStemMix`** — o app (SwiftUI), com *self-checks* por linha de comando.
- **`tools/make-app.sh`** — empacotamento, e um `.github/workflows/release.yml` que
  publica o `.zip` a partir da tag.

### As três decisões de arquitetura que valem ouro

Estas são as coisas que eu **não** quero redescobrir — e são exatamente o que a REA
deve extrair com evidência:

1. **Os efeitos vivem em buses compartilhados, como os sends de uma mesa real.**
   Assim, cortar um canal nunca mata a cauda do eco. Os buses podem alimentar uns aos
   outros, mas **nunca são ligados entre si no grafo de áudio**: cada retorno é
   entregue aos outros buses **pela memória, um buffer depois** (`BusPortal`), o que
   torna **impossível criar um laço de realimentação**.
2. **Soft takeover nos knobs.** Os knobs não são motorizados: depois de trocar de
   página, um knob só age quando alcança o valor atual. Isso é o que permite usar um
   controlador MIDI barato sem "pulos" de parâmetro.
3. **Nada corta o som tocando.** Qualquer mudança que pararia ou recabeaaria o motor
   de áudio (carregar plugin, posicionar stem) é **recusada durante a reprodução**,
   com um aviso (`refusedWhilePlaying`).

E uma decisão de dados: **o controlador MIDI é um perfil, dados puros**
(`ControllerProfile`: qual *control change* é qual knob, qual *note* é qual botão,
quais notas acendem os LEDs). A lógica de mix nunca "vê" o MIDImix — ela recebe
eventos abstratos.

---

## 2. Base legal e ética (obrigatório)

A ferramenta escolhida, **[REA / `rea-agents`](https://github.com/morluto/rea)**
(MIT), é explícita: *"REA is for lawful research, education, interoperability,
software preservation, and analysis of software **you own or are authorized to
examine**."* Ela não concede licença de Hopper/Ghidra/IDA, e não autoriza cópias
crackeadas.

**Neste projeto isso está limpo**, porque:

- O alvo, **DubStemMix, é MIT** — o código-fonte é público e a licença permite
  estudar, modificar e redistribuir (mantendo o aviso de copyright).
- Portanto a análise é de **software que eu estou autorizado a examinar**, e o
  objetivo é **interoperabilidade e portabilidade** (rodar no Linux e no Windows),
  não quebrar proteção nenhuma.
- **O modelo de separação não é redistribuído** (o próprio DubStemMix não o embarca,
  por licença): ele é baixado no primeiro uso, com consentimento. Vamos manter a
  mesma política.
- **Atribuição**: o que for derivado do DubStemMix leva crédito e o aviso MIT dele
  (arquivo `NOTICE`), exatamente como ele faz com terceiros.

---

## 3. Ferramenta: como o REA entra

**O que é:** REA (*Reverse Engineer Anything*) é um **servidor MCP** que conecta o
agente a ferramentas de inspeção de binários nativos, apps JavaScript/Electron,
assemblies .NET, sites e capturas de rede. Ele **devolve a evidência** de cada
conclusão (pseudocódigo, assembly, strings, chamadas), o que é o ponto: não é "achismo",
é rastreável.

**Requisitos (do README do projeto):**

| Item | Requisito |
|---|---|
| Node.js | 22.x (>= 22.19), 24.x (>= 24.11) ou 26+ **e** npm |
| Instalação | `npx rea-agents setup` (registra o MCP e as instruções de workflow) |
| Análise nativa profunda | **Hopper, Ghidra ou IDA** instalado |
| Análise estática de JavaScript/Electron | **Não exige** engine nativa |
| Linux | precisa de um `ps` compatível com `procps` no PATH do REA |

> Observação importante: o REA **analisa localmente** e o alvo deste roadmap
> (DubStemMix) é um binário **macOS/Apple Silicon**. Ou seja: a fase de análise
> nativa precisa de um Mac (ou de um Ghidra/IDA aberto sobre o binário) — e é a
> fase mais cara em tempo. A parte de **JavaScript/Electron não se aplica** aqui.

### Comandos úteis

```bash
# instalar/registrar no agente (uma vez)
npx rea-agents setup          # escolha o agente, revise, aprove, reinicie

# CLI avulsa, sem instalar globalmente
npx -y rea-agents@latest analyze-javascript-application /caminho/do/app --json

# instalar o comando `rea` para uso contínuo
npm install --global rea-agents
rea --help

# manter atualizado (o REA muda rápido)
rea update
```

---

## 4. Plano por fases

### Fase 0 — Preparação (curta)

- [ ] Documentar o estado atual: **esta mesa web é a especificação funcional.**
      Ela já define o que a versão nativa deve ter (28 knobs, 5 bandas, 3 delays,
      10 presets de sirene, Synare).
- [ ] Congelar o comportamento atual com os testes de áudio que já existem
      (impacto do Synare, cauda do delay, kills/onlys mudando o espectro) — viram
      **testes de aceitação da versão nativa**.
- [ ] Clonar o alvo para leitura: `git clone https://github.com/hroderik/DubStemMix`
      (é MIT; leitura direta do código é legítima e mais rápida que a REA para
      entender estrutura).
- [ ] Instalar Node 22+/24+ e rodar `npx rea-agents setup`.

### Fase 1 — Extrair a arquitetura de áudio (o núcleo do valor)

Foco: `BusPortal`, roteamento de buses, regras de tempo real.

- [ ] Levantar o **contrato do `BusPortal`**: como um bus entrega sinal a outro pela
      memória, um buffer depois, sem criar laço. Objetivo: reproduzir isso num motor
      portátil.
- [ ] Levantar a **ordem da cadeia de master** e onde entram sub/auto-wah.
- [ ] Levantar as **regras de tempo real** dos kernels em C (sem alocação, sem lock) —
      o que exatamente é proibido na thread de áudio.
- [ ] Extrair o **comportamento de cada kernel** do `DubDSP`: tape delay, plate reverb,
      spring reverb, phaser, flanger, sub, auto-wah (coeficientes, tempos, curvas).

### Fase 2 — Extrair a camada de controle

- [ ] O **soft takeover** do `MixController`: como ele compara posição do knob com o
      valor atual e a partir de quando passa a agir.
- [ ] O **formato do `ControllerProfile`** (dados puros) — para eu desenhar perfis
      para controladores baratos (ex.: Novation Launch Control XL, KORG nanoKONTROL).
- [ ] A regra **`refusedWhilePlaying`**: quais mudanças são recusadas tocando e como
      o aviso é apresentado.

### Fase 3 — Extrair a separação de stems

- [ ] O pipeline **`StemSplit`**: decodificação → *overlap-add* → **htdemucs_ft via
      ONNX Runtime** → montagem dos stems.
- [ ] Como é feito o **download e a verificação** do modelo no primeiro uso
      (não embarcar o modelo — manter).
- [ ] Como o resultado é conferido: o projeto **compara a soma dos stems com o mix
      original** — isso é um teste de sanidade, não um detalhe. Reproduzir.

### Fase 4 — Portar para multiplataforma

O alvo é macOS-only (Apple Silicon, `AVAudioEngine`). A graça é a **portabilidade**:

- [ ] Escolher o motor: **miniaudio** (um header C, roda em Linux/macOS/Windows) ou
      `RtAudio`/`PortAudio`, mantendo os kernels em C puros.
- [ ] Portar os kernels do `DubDSP` (C puro → já portável, é o que torna isso viável).
- [ ] Reescrever a camada de controle (o projeto usa SwiftUI) — candidatos:
      **C++ + Dear ImGui** (mesmo visual "gabinete desenhado" desta mesa web),
      ou **Tauri/Electron + Web Audio** (reaproveita o `index.html` **e** os kernels
      em C via WASM).
- [ ] **Rota mais barata:** compilar os kernels em C para **WASM**
      (`AudioWorklet`) e reaproveitar **este `index.html`** como interface. Isso dá
      *sample-accurate* no navegador, com o DSP em C — sem descartar nada do que já
      existe. **É a rota que eu recomendo começar por aqui.**

### Fase 5 — Empacotar e publicar

- [ ] Inspirar no `tools/make-app.sh` e no `.github/workflows/release.yml` do alvo:
      a versão vem da **tag do git**, o release publica o `.zip`, e **o app tem
      updater** que procura esse zip.
- [ ] Fazer o mesmo para Linux (`.AppImage`/`.deb`) e Windows (`.exe`/`.msi`).
- [ ] Manter o GitHub Pages atual (web) como a versão "abre e usa".

---

## 5. Entregáveis

| # | Entregável | Critério de pronto |
|---|---|---|
| 1 | Docs de arquitetura extraídos, **com evidência** | Um documento com os pontos da seção 1, cada um com a referência (arquivo/função/instrução) |
| 2 | Kernels DSP em C portáveis | Os 7 efeitos compilando em Linux/macOS/Windows |
| 3 | Motor de áudio multiplataforma | Toca arquivo com < 10 ms de latência, sem *dropouts* em 10 min |
| 4 | Buses com handoff de 1 buffer | Cortar um canal não mata a cauda; laço é impossível de criar |
| 5 | Soft takeover + perfis de controlador | Um controlador MIDI barato funciona sem pulos |
| 6 | Separação de stems | 4 stems (vocal/baixo/bateria/resto) com a soma batendo com o mix |
| 7 | Builds assinados | Linux, macOS e Windows, com updater |

---

## 6. Riscos e limites (honestidade antes de otimismo)

| Risco | Mitigação |
|---|---|
| **Roda num Mac?** A REA nativa precisa de Hopper/Ghidra/IDA sobre um binário ARM macOS | Muita coisa se resolve **lendo o fonte MIT**; a REA entra no que o fonte não explica (comportamento de binário, otimizações, detalhes de real-time) |
| **Reescrever a UI em C++** é semanas | Rota WASM (Fase 4) reaproveita o `index.html` inteiro |
| **Licença do modelo de stems** | Não embarcar; baixar no primeiro uso com consentimento (como o alvo faz) |
| **Escopo infinito** | A **spec é esta mesa web**: 28 knobs, 5 bandas, 3 delays, sirene, Synare. Nada além, na v1 nativa |
| **Perder o que já funciona** | O `index.html` **permanece** como produto principal. A versão nativa é um segundo alvo, não um substituto |

---

## 7. Próximo passo concreto

```
1. Clonar hroderik/DubStemMix e ler Sources/DubDSP + BusPortal   (1–2 dias)
2. Escrever docs/ARQUITETURA.md com o que foi aprendido          (1 dia)
3. Provar a rota: compilar UM kernel (tape delay) em WASM e
   tocar dentro deste index.html via AudioWorklet                (2–3 dias)
4. Se o passo 3 funcionar, a rota está validada e o resto é escala.
```

O passo 3 é o mais importante do roadmap inteiro: **ele decide a arquitetura de toda
a versão executável** por um custo de dois ou três dias. Se o kernel em C rodar dentro
do `index.html` via `AudioWorklet`, ganhamos DSP de qualidade (o mesmo do projeto
nativo) **sem abandonar** a interface que já está pronta, testada e bonita.

---

## Referências

- [morluto/rea](https://github.com/morluto/rea) — REA, o MCP de engenharia reversa (MIT)
- [rea.tools](https://rea.tools/) — site, guias ilustrados e casos de estudo
- [hroderik/DubStemMix](https://github.com/hroderik/DubStemMix) — o alvo/porta de referência
  (upstream: [yoanbernabeu/DubStemMix](https://github.com/yoanbernabeu/DubStemMix))
- [`docs/ROADMAP-HARDWARE.md`](ROADMAP-HARDWARE.md) — a outra frente: a mesa física
