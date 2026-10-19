# 🔧 ROADMAP — Versão física (hardware)

> **Status:** planejamento. Nada comprado, nada soldado ainda.
> **Meta:** uma mesa de dub **de verdade**, que liga na tomada e no amplificador,
> com sirene, Synare, crossover e delay — **começando por R$ 100 em componentes**.
> **Relação com o software:** este `index.html` é a **especificação**. Todo controle
> da mesa física tem um controle equivalente aqui, com a mesma faixa de valores
> (seção 2). A mesa física não inventa nada: ela **materializa** o que já está testado.

---

## 1. Por que fazer a versão física

A versão web já faz o som. O que ela **nunca** vai fazer:

| Limite da versão web | O que o hardware resolve |
|---|---|
| Só toca arquivo — **não tem entrada de áudio** | Jack de entrada: você pluga o celular, o mic, a guitarra, o dubplate |
| Latência e *jitter* do navegador + sistema operacional | O sinal passa por um circuito: atraso é o do capacitor, não o do agendador do SO |
| Depende de computador ligado, navegador, servidor local | Liga na tomada. Sem tela, sem atualização, sem `localhost` |
| Não dá para levar para o paredão / a festa sem um notebook | Cabe numa caixa de madeira, roda na bateria de 9 V |
| O "louco" do dub é abstrato (knob na tela) | Você **gira o potenciômetro**, sente o *click* do switch, vê o VU analógico dançar |

### A regra de ouro desta frente

> **Primeiro fazer um canal funcionar de ponta a ponta. Depois multiplicar.**

Um canal de áudio que entra, passa pelos filtros, sai no amplificador — mesmo
sem sirene, sem Synare, sem delay — já é um instrumento. Tudo o mais é repetição
e empilhamento. Quem tenta montar as 28 funções de uma vez desiste na terceira
semana com a bancada cheia de placas pela metade.

---

## 2. A especificação: o software vira datasheet

Esta tabela é o **contrato** entre as duas versões. Se o hardware tiver esses
controles, nesses valores, ele faz os mesmos sons.

| Controle na tela | Faixa real no app | Componente físico | Observação de projeto |
|---|---|---|---|
| VOL CH1 / CH2 (deck) | 0–100 % | potenciômetro **A10k** (log) | Curva log = o ouvido, não o voltímetro |
| ISO (baixo) | 0–200 % (`val×2`) | potenciômetro + **shelf** de graves | Precisa de ganho > 1: tem que ser ativo (op-amp) |
| CUTOFF (filtro por canal) | 100 Hz – 20 kHz | potenciômetro **B100k** + **TL072** (Sallen-Key) | Filtro de 2ª ordem, ressonância fixa |
| Crossover SUB | < 80 Hz | **LPF 2ª ordem** (TL072) | 5 bandas — seção 4 |
| Crossover BASS | 80–250 Hz | **BPF** (LPF + HPF, TL072) | |
| Crossover L-MID | 250 Hz – 1,2 kHz | **BPF** (TL072) | |
| Crossover H-MID | 1,2–5 kHz | **BPF** (TL072) | |
| Crossover HIGH | > 5 kHz | **HPF 2ª ordem** (TL072) | |
| X-FADER | 0–100 % (lei de cos/sen) | potenciômetro **B100k** duplo | Lei de *cos/sen* dá ganho constante no meio |
| X-FADER | volta ao centro | botão + reset | No hardware: um *center detent* |
| SirenPitch | 50 – 1500 Hz | potenciômetro **B100k** | Define o VCO — seção 3 |
| SirenRate | 0,01 – 25 Hz | potenciômetro **B1M** (lento) + **CD4046** | LFO muito lento = o "uivo" do dub |
| Siren VOL | 0–100 % | potenciômetro **A10k** | |
| Siren DELAY TIME | 0,05 – 1,20 s | potenciômetro **B100k** no **PT2399** | ~1,2 s no limite da qualidade do chip |
| Siren DELAY FB | 0 – 90 % | potenciômetro **A100k** | 90 % é o teto — acima vira auto-oscilação |
| Siren DELAY WET | 0–100 % | potenciômetro **A10k** | *mix* seco/molhado |
| SynTune | 40 – 1000 Hz | potenciômetro **B100k** | Afinação do tambor |
| SynDecay | 0,05 – 1,5 s | potenciômetro **B1M** + **LM13700** | Decaimento do envelope |
| SynTone | 100 – 6000 Hz | potenciômetro **B100k** + **TL072** | Filtro do ruído do ataque |
| SynNoise | 0–100 % | potenciômetro **A10k** | Quantidade de ruído no ataque |
| SynSweep | 0–100 % | potenciômetro **A10k** | Quanto o "pew" cai de tom |
| SynSpace | 0–100 % | potenciômetro **A10k** | Reverb/envelope à parte |
| 3 delays (canal, sirene, Synare) | DTime 0,05–1,2 s / FB 0–90 % / Wet 0–100 % | **2× PT2399** ou **1 PT2399 + BBD** | Seção 5 |
| MASTER | 0–100 % | potenciômetro **A10k** | Último antes da saída |
| KILL / ONLY (5 botões) | liga/desliga | **CD4066** (chave analógica) ou relé | Chave analógica é silenciosa e barata |
| VU / ponteiro | 0–100 % | **LM3915** ou **Arduino + barra de LED** | O ponteiro analógico é o charme |

**Conclusão prática:** o hardware precisa de **~24 potenciômetros**, **5–6 filtros
ativos**, **1 VCO de sirene**, **1 voz de tambor** e **2 delays**. Nada exótico.
Todo chip desta lista é vendido no Brasil por unidade, em loja de eletrônica.

---

## 3. Arquitetura proposta

```
   ENTRADAS                          NÚCLEO                              SAÍDA
  ┌──────────┐   ┌────────────┐   ┌──────────────────┐   ┌──────────┐   ┌─────────┐
  │ Jack CH1 ├──►│ Pré + ISO  ├──►│  CROSSOVER 5 vias│──►│ Somador  ├──►│ MASTER  │
  │ (6,35 mm)│   │ (NE5532 +  │   │  SUB/BASS/LMID/  │   │ (TL072)  │   │ A10k    │
  └──────────┘   │  shelf)    │   │  HMID/HIGH       │   │          │   └────┬────┘
  ┌──────────┐   └────────────┘   │  (TL072 x3)      │   └────┬─────┘        │
  │ Jack CH2 ├──►   idem         └────────┬─────────┘        │              │
  └──────────┘                            │                  │         ┌────▼────┐
                                          │                  │         │  VU     │
   ┌──────────────────────────────────────┴──────────────────┴────┐    │ LM3915  │
   │                      MATRIZ DE ENTRADA                         │    └─────────┘
   │  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐    │         │
   │  │ SIRENE   │   │ SYNARE   │   │ DELAY    │   │ KILL/    │    │    ┌────▼────┐
   │  │ CD4046   │   │ LM13700  │   │ PT2399   │   │ ONLY     │    │    │  SAÍDA  │
   │  │ VCO+LFO  │   │ osc+ruído│   │ x2       │   │ CD4066   │    │    │ L/R     │
   │  │ ─────────│   │ +envelope│   │ ──────── │   │ ──────── │    │    └─────────┘
   │  │ mix do   │   │ mix do   │   │ mix seco/│   │ corta    │    │
   │  │ delay    │   │ delay    │   │ molhado  │   │ banda    │    │
   │  └──────────┘   └──────────┘   └──────────┘   └──────────┘    │
   └───────────────────────────┬───────────────────────────────────┘
                               │
                        ┌──────▼──────┐
                        │  CONTROLE   │  Arduino Nano / RP2040:
                        │  (opcional) │  VU por software, MIDI, LED,
                        └─────────────┘  presets por relé
```

**Sinal em palavras:** cada canal entra, ganha graves e corte de agudos, e é
**dividido em 5 bandas**. Sirene, Synare e delay entram na matriz e podem ser
mandados para qualquer banda. A matriz soma tudo, o MASTER atende o volume e o VU
mostra o nível. Simples — e é exatamente o que o `index.html` faz no grafo Web Audio.

---

## 4. Crossover de 5 bandas (o coração da mesa)

O crossover é o que faz "dub" soar dub: cortar a banda é a *linguagem*. Aqui não é
um knob que "abaixa" uma banda no software — é um circuito que **tira a banda**.

```
              ┌───────────  LPF 80 Hz  ──────────►  SUB
              │
entrada ──────┤   HPF 80   ┌──  LPF 250   ──────►  BASS
              └───────────►│
                           │   HPF 250  ┌──  LPF 1k2  ──►  L-MID
                           └───────────►│
                                        │  HPF 1k2 ┌── LPF 5k ─►  H-MID
                                        └─────────►│
                                                   └── HPF 5k ──►  HIGH
```

- Filtros **Sallen-Key de 2ª ordem** com **TL072** (um TL072 = 2 filtros; o
  crossover completo usa ~3 TL072). Butterworth, Q ≈ 0,707.
- Um **JFET ou CD4066** em série com cada banda realiza o KILL/ONLY: em vez de
  mexer no ganho, ele **abre o caminho** da banda. É assim que o corte fica limpo.
- Cada banda tem um **trimpot** no ganho para casar os níveis (as bandas não saem
  com a mesma energia — a banda de agudos sai bem mais fraca).

---

## 5. Sirene e Synare — o caráter da mesa

### Sirene (`CD4046`)

O **CD4046** é um PLL com **VCO interno**: é literalmente um oscilador controlado
por tensão pronto, por menos de R$ 10. `SirenPitch` é a tensão de controle,
`SirenRate` é o **LFO** que varre essa tensão. O LFO precisa chegar a **0,01 Hz**
(100 segundos por ciclo) — isso pede um capacitor grande e um B1M, e é o que faz o
"uivo longo" do dub, que nenhum sintetizador moderno faz bem.

### Synare / Space Drum (`LM13700` + ruído)

A batida clássica é **três sinais somados**:

1. **Oscilador triangular** com envelope de *pitch* (o "pew"): o `SynSweep`
   controla quanto a afinação cai. O **LM13700** (amplificador de
   transcondutância duplo) faz VCO **e** VCA com um chip só.
2. **Ruído branco** no ataque: um transistor com a junção base-emissor
   reversamente polarizada é a fonte de ruído mais barata que existe. Passa pelo
   filtro `SynTone`.
3. **Envelope de amplitude**: o `SynDecay` (0,05–1,5 s) é o tempo do capacitor de
   envelope. Duas etapas (ataque rápido, queda longa) = o tambor.

> **Detalhe que faz diferença:** no app, Synare e Sirene têm **delay próprio**.
> No hardware isso significa **dois envios independentes** para o mesmo PT2399 —
> ou seja, cada um tem seu potenciômetro de WET. É por isso que o app tem 28 knobs
> e não 12.

---

## 6. Delay (`PT2399`)

O **PT2399** é o chip de delay digital mais barato do mundo (~R$ 8), usado em
milhares de pedais. Ele é **lofi**: 12 bits, ~40 kHz de amostragem, com *clock*
ruidoso. **Isso não é um defeito — é o som.** O delay de dub é sujo, escuro e com
cauda. Pelo datasheet, ele chega a ~1,2 s com qualidade já degradada — que é
justamente onde o app deixa o `DTime` (0,05–1,20 s).

- **FB 0–90 %**: acima de ~95 % ele vira oscilador. O app limita em 90 % de
  propósito; o hardware usa o mesmo teto (um resistor em série no potenciômetro
  de realimentação, para que **não exista** posição de auto-oscilação).
- **Filtro na realimentação**: um LPF simples (RC ou TL072) escurece cada repetição
  — é o que faz a cauda "afundar no lodo" em vez de guinchar.
- **3 delays no app** = no hardware, um ponto de soma com 3 potenciômetros de
  envio e 1 mistura. Se um só PT2399 não der conta do tempo, dois PT2399 em série
  (um atrasa, o outro atrasa o atraso) dobram o tempo.

---

## 7. Lista de materiais (BOM) e os três patamares

Preços **estimados** em loja brasileira, 2026, por unidade (não comprando 100 peças).
Valem como ordem de grandeza: quem montar vai economizar comprando kit e reusando
sucata, ou gastar mais comprando tudo em loja de bairro.

### Patamar 1 — "R$ 100" — Mono, protoboard/perfboard, caixa de plástico

**O que funciona:** 1 canal de entrada, crossover de 5 bandas com kill, sirene com
VCO + LFO, Synare, **1 delay** compartilhado, MASTER, VU de LED simples.

| Item | Qtd | Preço un. | Subtotal |
|---|---|---|---|
| PT2399 (delay) | 1 | R$ 8 | R$ 8 |
| TL072 (filtros, pré, somador) | 3 | R$ 3 | R$ 9 |
| NE5532 (pré de entrada, baixo ruído) | 1 | R$ 6 | R$ 6 |
| CD4046 (VCO da sirene) | 1 | R$ 6 | R$ 6 |
| LM13700 (Synare) | 1 | R$ 18 | R$ 18 |
| CD4066 (kill/only) | 1 | R$ 3 | R$ 3 |
| Potenciômetros (A10k, A100k, B100k, B1M) | 12 | R$ 3 | R$ 36 |
| Knobs | 12 | R$ 1,50 | R$ 18 |
| Jack 6,35 mm mono | 3 | R$ 5 | R$ 15 |
| Conjunto de resistores + capacitores | kit | R$ 25 | R$ 25 |
| Perfboard / protoboard | 2 | R$ 8 | R$ 16 |
| Fonte 9 V + conector | 1 | R$ 18 | R$ 18 |
| Fonte de ruído (transistor + 2 resistores) | — | R$ 2 | R$ 2 |
| LED + resistores + fios | kit | R$ 12 | R$ 12 |

> Os potenciômetros, knobs e jacks são **metade do custo**. Não tem como fugir:
> é o preço de ter dedos girando coisas em vez de um mouse.
> **Total realista: R$ 150–200** com impostos e frete de uma compra só.
> Os R$ 100 são atingíveis **reusando** potenciômetros de sucata, jacks velhos e
> uma fonte de 9 V de roteador já em casa — que é o cenário de quem monta em casa.

### Patamar 2 — "PCB próprio" — Estéreo, placa de circuito impresso

| Item | Qtd | Preço un. | Subtotal |
|---|---|---|---|
| Base do patamar 1 | — | — | ~R$ 180 |
| PCB (JLCPCB/PCBWay, 5 placas, frete incluído) | 5 | ~R$ 60 | R$ 60 |
| 2º canal completo (NE5532 + TL072 + pots + jacks) | — | — | R$ 90 |
| 2º PT2399 (delay do Synare separado) | 1 | R$ 8 | R$ 8 |
| Arduino Nano (clone) + barra de LED / OLED | 1 | R$ 30 | R$ 30 |
| MDF/madeira + parafusos + tinta | — | — | R$ 45 |
| **Total** | | | **~R$ 400** |

Aqui a mesa já é **estéreo, com PCB**, e o Arduino entra para: VU por software,
**presets de sirene por relé**, e **MIDI** (o `index.html` já é a referência de
comportamento — o firmware só reproduz).

### Patamar 3 — "mesa de verdade" — Chassi de metal, painel serigrafado

Chassi de alumínio dobrado, painel com serigrafia, potenciômetros de qualidade
(Alpha/Bourns), jacks Neutrik, **delay BBD real** (MN3207 + MN3102, o *chip* do
delay analógico clássico) em vez de PT2399, VU **de ponteiro** de verdade,
alimentação ±12 V com 78L12/79L12, transformador.

> **Custo: R$ 900–1.500.** É outro projeto. Vale se o objetivo é ter o
> equipamento na estante, e não aprender eletrônica de áudio.

---

## 8. Painel, madeira e estética

O `index.html` tem 8 temas e um gabinete que mistura **madeira + metal**. A mesa
física copia isso porque o projeto visual **já foi testado e aprovado**:

- **Laterais de madeira** (MDF ou madeira de verdade): 10–26 mm, como no app.
- **Painel de metal**: chapa de alumínio 2 mm, ou aço pintado.
- **Bordas rebaixadas** e parafusos aparentes.
- **Knobs metálicos com indicador branco** (o app desenha exatamente isso) e os
  **push buttons redondos** (SIRENE e SWEEP) — no app o botão fica **cinza** e vira
  **vermelho** quando ligado; no hardware, a cor vem de um **anel de LED** em volta
  do botão, com o mesmo comportamento.
- **VU analógico** (ponteiro) é o item que mais "vende" a mesa na foto. Se o
  orçamento não permitir, a barra de LED com **LM3915** dá o mesmo movimento.

> **Dica de execução:** imprimir o painel em papel, colar na chapa e furar pelos
> furos do papel. Depois tirar o papel e usar a chapa como gabarito. É assim que
> se faz painel sem CNC.

---

## 9. PCB no KiCad — do esquemático à placa

Ferramenta: **KiCad** (livre, roda em Linux/macOS/Windows). O PCB abre a porta do
patamar 2 e reduz o preço do patamar 1 (uma placa sai mais barata que 40 fios de
perfboard), então vale fazer em paralelo ao protótipo.

1. **Esquemático por bloco.** Um bloco por função: pré+ISO, crossover (×1 canal),
   sirene, Synare, delay, matriz, MASTER, alimentação, controle. Cada bloco é
   desenhado e **testado na protoboard** antes de virar placa — nunca o contrário.
2. **Atribuição de encapsulamento** (footprints): potenciômetros, jacks e
   interruptores definem **o tamanho da placa e o espaçamento do painel**. Escolher
   o footprint é decidir o painel. Fazer isso antes de rotear.
3. **Plano de terra.** Áudio é analógico: **plano de terra único e contínuo**, com
   o caminho de retorno de cada bloco voltando ao mesmo ponto. Terra mal feito =
   chiado e oscilação. Separar as trilhas de alimentação e **desacoplar cada CI**
   (100 nF cerâmico + 10 µF eletrolítico ao lado do chip).
4. **Duas camadas** bastam. Áudio não precisa de 4 camadas. Trilhas largas no
   caminho do sinal (0,4 mm+) e mais largas na alimentação.
5. **Verificação de regras (DRC)** e conferência do esquemático contra o
   `index.html` (seção 2) — cada controle da placa tem que ter equivalente na tela.
6. **Fabricação**: JLCPCB/PCBWay aceitam o `.zip` de Gerbers do KiCad. 5 placas de
   ~100×100 mm, 2 camadas, custam poucos dólares + frete. Prazo: 1–3 semanas.
7. **Silkscreen**: usar a camada de serigrafia para escrever no painel os mesmos
   nomes do app (`ISO`, `CUTOFF`, `SIRENE`, `SWEEP`, `SYNARE`, `SUB`…).

---

## 10. Firmware (Arduino Nano / RP2040) — opcional

O firmware **não faz áudio**. Ele faz o que o software faz de "inteligente":
medição, memória e comunicação. Se não quiser firmware, o patamar 1 funciona sem ele.

| Função | Como |
|---|---|
| VU | Ler o nível na saída e acender a barra de LED / mover o ponteiro |
| Presets de sirene | Os **10 presets nomeados** do app viram 10 posições de memória; o Arduino grava os valores e aciona relés/optoacopladores para "girar os knobs" |
| MIDI | Entrada MIDI (DIN ou USB) para controlar a mesa de um controlador externo |
| Sincronia | Detectar o tempo (como o app faz) e mandar o LFO da sirene entrar no compasso |
| Auto-desligamento | Desligar a alimentação após X minutos sem sinal |

O firmware é onde o software e o hardware se encontram: o
mesmo comportamento documentado do app vira a especificação do firmware.

---

## 11. Fases

- [ ] **Fase 0 — bancada.** Comprar chips + potenciômetros. Nada soldado ainda.
- [ ] **Fase 1 — um canal.** Jack → pré → crossover → MASTER → saída. Ouvir.
- [ ] **Fase 2 — o caráter.** Sirene (CD4046) e depois Synare (LM13700 + ruído).
- [ ] **Fase 3 — o espaço.** Delay PT2399 com FB e filtro na realimentação.
- [ ] **Fase 4 — a mesa.** Kill/ONLY, VU, segundo canal, X-FADER, caixa de madeira.
- [ ] **Fase 5 — a placa.** Esquemático e PCB no KiCad, mandar fabricar.
- [ ] **Fase 6 — a cabeça.** Firmware: VU, presets, MIDI.

Cada fase entrega um instrumento **tocável**. Se o projeto parar na Fase 3, ainda
existe uma mesa de dub com sirene e Synare — não uma caixa de peças.

---

## 12. Riscos e limites (honestidade antes de otimismo)

| Risco | Mitigação |
|---|---|
| **R$ 100 não cobrem uma réplica 1:1 de 28 knobs** | O patamar 1 é **mono e reduzido**, e assume reuso de sucata. Fingir que dá para fazer tudo com R$ 100 é enganar quem vai comprar |
| **Ruído, chiado e oscilação** no áudio analógico | Desacoplar cada CI, plano de terra único, trilhas curtas, e testar bloco por bloco |
| **Comprar tudo de uma vez e não montar** | A Fase 0/1 é minúscula de propósito. Só comprar mais depois de ouvir a Fase 1 |
| **PT2399 curto demais** para o delay de 1,2 s | Dois PT2399 em série; ou aceitar ~600 ms e usar como *slap*, que é o som do dub raiz |
| **LM13700 difícil de achar / caro** | Alternativas: VCO com CD4046, VCA com CD4066 + envelope RC. Perde qualidade, ganha disponibilidade |
| **Potenciômetro errado (A/B/curva)** mata a sensação | A coluna "componente físico" da seção 2 é obrigatória. O app define as faixas |
| **Polaridade e tensão erradas queimam chips** | Checar o datasheet duas vezes, 9 V antes de ±12 V, e usar soquete nos CIs |

---

## 13. Próximo passo concreto

```
1. LISTA  — fechar a BOM do patamar 1 e conferir preço em UMA loja só   (1 noite)
2. PEDIR  — comprar APENAS a Fase 1: TL072, NE5532, pots, 1 jack,
            resistores/caps, protoboard. Nada de PT2399/LM13700 ainda.   (1 dia)
3. OUVIR  — montar entrar -> crossover -> somador -> master.
            Ajustar níveis com trimpot. Fazer o KILL de uma banda.       (1 fim de semana)
4. GRAVAR — gravar um áudio de 30 s da Fase 1 e comparar de ouvido com
            o index.html com os mesmos ajustes. É o teste da spec.       (1 tarde)
5. SÓ ENTÃO comprar sirene, Synare e delay.
```

O passo 4 é o mais importante desta frente: **se a Fase 1 soar como o `index.html`
com os mesmos ajustes, a especificação está validada** e o resto é repetir o que
deu certo. Se não soar, descobrimos isso gastando R$ 60 em vez de R$ 400.

---

## Referências

- `rust/` — a outra frente: a versão executável nativa (Rust/Tauri)
- [Datasheet PT2399](https://www.google.com/search?q=PT2399+datasheet) — delay digital lofi
- [Datasheet CD4046](https://www.google.com/search?q=CD4046+datasheet) — PLL e VCO
- [Datasheet LM13700](https://www.google.com/search?q=LM13700+datasheet) — OTA: VCO e VCA
- [Datasheet TL072](https://www.google.com/search?q=TL072+datasheet) / [NE5532](https://www.google.com/search?q=NE5532+datasheet) — op-amps dos filtros
- [KiCad](https://www.kicad.org/) — esquemático e PCB, livre e multiplataforma
- [JLCPCB](https://jlcpcb.com/) / [PCBWay](https://www.pcbway.com/) — fabricação das placas
