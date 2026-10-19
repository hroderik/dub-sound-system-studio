# 🔧 ROADMAP — Versão física (hardware)

> **Status:** planejamento. Nada comprado, nada soldado ainda.
> **Meta realista:** uma mesa de dub **de verdade**, que liga na tomada e no amplificador,
> começando pelo **primeiro canal audível por ~R$ 350–450** (comprando tudo novo) —
> a mesa completa e estéreo fica na casa dos **R$ 2.500–3.500**.
> **Relação com o software:** o `index.html` é a **especificação**. Todo controle da
> mesa física tem equivalente no app, com a mesma faixa de valores (seção 2) — e o
> app continua evoluindo mais rápido que a bancada, o que é bom: cada função nova
> lá é um circuito a menos pra errar aqui.

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
e empilhamento. Quem tenta montar as 36 funções da tela de uma vez desiste na terceira
semana com a bancada cheia de placas pela metade.

---

## 2. A especificação: o software vira datasheet

O app tem hoje **36 knobs**, 1 X-FADER e ~20 botões (kills, onlys, TIRAR AGULHA,
DISCO REVERSO, SIRENE, SWEEP...). A tabela abaixo é o **contrato** entre as duas
versões — os blocos principais. Se o hardware tiver esses controles, nesses valores,
ele faz os mesmos sons.

| Controle na tela | Faixa real no app | Componente físico | Observação de projeto |
|---|---|---|---|
| VOL CH1 / CH2 | 0–100 % | potenciômetro **A10k** (log) | Curva log = o ouvido, não o voltímetro |
| FILTER por canal | LP 100 Hz–20 kHz ↔ HP 20 Hz–20 kHz | potenciômetro **B100k** + **TL072** (Sallen-Key) | Filtro de 2ª ordem, bipolar (centro = aberto) |
| PAN por canal | L 100 – C – R 100 | potenciômetro duplo **MN** ou blend | Soma/mistura das vias L/R |
| Ganho por banda (×5) | 0–200 % | potenciômetro **B100k** + shelf ativo | Precisa de ganho > 1: tem que ser ativo (op-amp) |
| Crossover (5 bandas) | < 80 / 80–250 / 250–1k2 / 1k2–5k / > 5k | **LPF/BPF/HPF 2ª ordem** (TL072 ×3) | Seção 4 — o coração da mesa |
| KILL / ONLY | liga/desliga por banda | **CD4066** (chave analógica) | Chave analógica é silenciosa e barata |
| X-FADER | 0–100 % (lei de cos/sen) | fader deslizante **duplo B100k** | *Center detent* segura o centro no lugar |
| BASS BOOST | +12 dB < 110 Hz | shelf ativo (TL072) | Somável ao TREMEDEIRA |
| 💥 TREMEDEIRA | graves + distorção "suja" | shelf extra + diodo/clipper + limitador | No app é wave-shaping; no hardware, saturador |
| SirenPitch | 50–1500 Hz | potenciômetro **B100k** no **CD4046** | Define o VCO — seção 5 |
| SirenRate | 0,01–25 Hz | potenciômetro **B1M** + **CD4046** | LFO muito lento = o "uivo longo" do dub |
| SynTune / Sweep | 40–1000 Hz / 0–100 % | potenciômetros **B100k** + **LM13700** | VCO do tambor + profundidade do "pew" |
| SynDecay / Tone / Noise / Space | 0,05–1,5 s / 100 Hz–6 kHz / 0–100 % | potenciômetros + **LM13700** + ruído | Envelope, filtro do ruído, misturas |
| Delays (TIME/FB/WET) | 0,05–1,20 s / 0–90 % / 0–100 % | **PT2399** + filtro na realimentação | Um chip, três envios (seção 6) |
| REVERB de mola | 0–100 % | **tank de mola** + driver/recovery | Opcional e "uau" — ou sucata de amplificador |
| MASTER | 0–100 % | potenciômetro **A10k** | Último antes da saída |
| Gravar / Converter | — | (não existe no hardware) | Gravar é do mundo digital; a saída vai pra mesa/DAW |
| TAP TEMPO / MODE (12) | 0,05–1,20 s | (firmware + digipot) | Recurso digital; no hardware puro, só o knob TIME |

**Conclusão prática:** a mesa física **completa** precisa de ~36 potenciômetros,
~8 filtros ativos, 1 VCO de sirene, 1 voz de tambor, 2 PT2399 e 1 CD4066 (ou 2).
Nada exótico — mas não é barato. Por isso as **fases** (seção 11) entregam som antes
de a mesa ficar completa.

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

### Ligação elétrica e alimentação (o que costuma dar ruído)

- **Fonte:** 9 V único para o primeiro protótipo (a maioria dos chips da lista vai de
  5 a 15 V). Só migre para **±12 V com 7812/7912** quando entrar o VU de ponteiro
  e o segundo canal. Nunca ligue os dois PT2399 e o LM13700 no mesmo fio fino de
  protoboard: trilha de alimentação larga, capacitância de sobra.
- **Terra estrela:** um único ponto de terra (perto do jack de saída) onde todos os
  blocos se encontram. Terra em corrente (cadeia) = zumbido de 60 Hz garantido.
- **Desacoplamento:** 100 nF cerâmico + 10 µF eletrolítico em **cada** CI, o mais
  colado possível do pino de alimentação.
- **Blindagem:** jacks com a carcaça no terra; cabo do tank de mola é antena — mantenha
  longe dos potenciômetros e passe perto do chassi.
- **Digital × analógico:** se usar Arduino, o terra do microcontrolador entra no mesmo
  ponto estrela, e o cabo USB/FTDI não fica conectado durante o uso (senão, ruído).

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
- Um **CD4066** em série (ou shunt) com cada banda realiza o KILL/ONLY: em vez de
  mexer no ganho, ele **abre o caminho** da banda. É assim que o corte fica limpo.
- Cada banda tem um **trimpot** no ganho para casar os níveis (as bandas não saem
  com a mesma energia — a banda de agudos sai bem mais fraca).

> **Ponte com o app:** na tela, cada banda tem fader de ganho + kill + LED; no
> Web Audio, isso é `BiquadFilterNode` ×2 + `GainNode` + `AnalyserNode`. A matriz
> de bandas do app (com as frequências de corte na seção 2) é o diagrama do PCB.

---

## 5. Sirene e Synare — o caráter da mesa

### Sirene (`CD4046`)

O **CD4046** é um PLL com **VCO interno**: é literalmente um oscilador controlado
por tensão pronto. `SirenPitch` é a tensão de controle, `SirenRate` é o **LFO** que
varre essa tensão. O LFO precisa chegar a **0,01 Hz** (100 segundos por ciclo) —
isso pede um capacitor grande e um B1M, e é o que faz o "uivo longo" do dub, que
nenhum sintetizador moderno faz bem.

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
> ou seja, cada um tem seu potenciômetro de WET. É por isso que o app tem 36 knobs
> e não 12.

---

## 6. Delay (`PT2399`)

O **PT2399** é o chip de delay digital mais barato do mundo, usado em milhares de
pedais. Ele é **lofi**: 12 bits, ~40 kHz de amostragem, com *clock* ruidoso.
**Isso não é um defeito — é o som.** O delay de dub é sujo, escuro e com cauda.
Pelo datasheet, ele chega a ~1,2 s com qualidade já degradada — que é justamente
onde o app deixa o `REPEAT RATE` (0,05–1,20 s).

- **FB 0–90 %**: acima de ~95 % ele vira oscilador. O app limita em 90 % de
  propósito; o hardware usa o mesmo teto (um resistor em série no potenciômetro
  de realimentação, para que **não exista** posição de auto-oscilação).
- **Filtro na realimentação**: um LPF simples (RC ou TL072) escurece cada repetição
  — é o que faz a cauda "afundar no lodo" em vez de guinchar.
- **3 delays no app** = no hardware, um ponto de soma com 3 potenciômetros de
  envio e 1 mistura. Se um só PT2399 não der conta do tempo, dois PT2399 em série
  (um atrasa, o outro atrasa o atraso) dobram o tempo.
- **TAP TEMPO / MODE (12 divisões)** são digitais: no hardware puro, o controle é
  o knob; tap tempo ficaria pro firmware (Arduino + digipot), se um dia valer.

---

## 7. Lista de materiais (BOM) e os três níveis

Preços em loja brasileira, **2026, por unidade** (sem comprar 100 peças; com frete
e impostos de um pedido único como ordem de grandeza). Reusar sucata (potenciômetros
de rádios velhos, jacks, caixa, fonte de roteador) corta 30–40 % do total.

### Nível 1 — "primeiro som" — Mono, protoboard, 1 canal

**O que funciona:** 1 canal de entrada, pré com shelf, crossover de 5 vias com kill
de uma banda, somador e MASTER. Já é um instrumento pra ouvir e testar a spec.

| Item | Qtd | Preço un. | Subtotal |
|---|---|---|---|
| TL072 (filtros + somador) | 3 | R$ 8 | R$ 24 |
| NE5532 (pré de entrada) | 1 | R$ 10 | R$ 10 |
| CD4066 (kill/only) | 1 | R$ 6 | R$ 6 |
| Potenciômetros 16 mm (A/B) | 8 | R$ 15 | R$ 120 |
| Knobs | 8 | R$ 6 | R$ 48 |
| Jack 6,35 mm de painel | 3 | R$ 10 | R$ 30 |
| Chaves de alavanca (mata banda) | 2 | R$ 12 | R$ 24 |
| Kit resistores 1 % + capacitores | kit | R$ 70 | R$ 70 |
| Protoboard / perfboard | 2 | R$ 15 | R$ 30 |
| Fonte 9–12 V 1 A + conector | 1 | R$ 35 | R$ 35 |
| Caixa provisória (plástico/MDF) | 1 | R$ 40 | R$ 40 |
| Fios, parafusos, soquetes de CI | kit | R$ 25 | R$ 25 |
| **Total** | | | **≈ R$ 460** |

> Comprando tudo novo dá **~R$ 400–500**. Reusando potenciômetros, jacks, caixa e
> fonte de sucata, cai para **~R$ 250–300**.

### Nível 2 — "mesa completa mono" — Sirene + Synare + delay + VU + caixa de madeira

| Item | Qtd | Preço un. | Subtotal |
|---|---|---|---|
| Base do Nível 1 | — | — | ~R$ 460 |
| CD4046 (VCO da sirene) | 1 | R$ 12 | R$ 12 |
| LM13700 (Synare) | 1 | R$ 40 | R$ 40 |
| PT2399 (delay) | 1 | R$ 20 | R$ 20 |
| CD4066 extra (kills das outras bandas) | 1 | R$ 6 | R$ 6 |
| Potenciômetros extras (sirene 3 + Synare 6 + delay 3 + kills 5) | 17 | R$ 15 | R$ 255 |
| Knobs extras | 17 | R$ 6 | R$ 102 |
| Chaves (SIRENE, SWEEP, etc.) | 4 | R$ 12 | R$ 48 |
| Kit R/C extra (VCO, envelopes, filtro do delay) | kit | R$ 40 | R$ 40 |
| Perfboards / placas perfuradas boas | 3 | R$ 18 | R$ 54 |
| Caixa de madeira (MDF cortado) + tinta | 1 | R$ 90 | R$ 90 |
| VU de LED (módulo pronto ou LM3914/15) | 1 | R$ 30 | R$ 30 |
| Fonte 12 V 1 A (ou trafo + 7812) | 1 | R$ 45 | R$ 45 |
| **Total** | | | **≈ R$ 1.200** |

> Faixa realista: **R$ 1.000–1.300** novo; **~R$ 700–850** com reuso pesado de sucata.
> Neste nível a mesa já tem os três delays, sirene com LFO, Synare e kill em todas
> as bandas — é a mesa completa em mono.

### Nível 3 — "a mesa de verdade" — Estéreo, PCB próprio, chassi de metal

| Item | Preço |
|---|---|
| Base do Nível 2 | ~R$ 1.200 |
| 2º canal completo (chips + pots + knobs + jack) | R$ 165 |
| PCB fabricado (ex.: 5 placas 100×100 mm, frete incluído) | R$ 120 |
| Arduino Nano (clone) + componentes | R$ 35 |
| VU **de ponteiro** importado (×2) | R$ 260 |
| Reverb **de mola** (tank + driver/recovery) | R$ 200 |
| Fader deslizante duplo (X-FADER) | R$ 45 |
| Chassi de alumínio dobrado + painel serigrafado | R$ 350 |
| Trafo + ±12 V (7812/7912) + dissipador | R$ 110 |
| **Total** | **≈ R$ 2.500** |

> Faixa realista: **R$ 2.500–3.500**, com erros, reposição e frete. VU de ponteiro e
> mola podem sair da conta (sucata de amplificador velho) — sem eles, é ~R$ 2.100.
> É outro projeto: vale se o objetivo é ter o equipamento na estante.

---

## 8. Painel, madeira e estética

O `index.html` tem 16 temas e um gabinete que mistura **madeira + metal**. A mesa
física copia a linguagem visual do app porque o projeto já está desenhado e testado:

- **Laterais de madeira** (MDF ou madeira de verdade). Painel de metal: chapa de
  alumínio 2 mm, ou aço pintado.
- **Knobs metálicos com indicador branco** (o app desenha exatamente isso — é o
  gabarito de posição dos furos).
- **Push buttons redondos** (SIRENE e SWEEP): no app, cinza desligado e vermelho
  ligado; no hardware, um **anel de LED** em volta do botão com o mesmo comportamento.
- **VU**: o de ponteiro é o que mais "vende" a mesa na foto; se o orçamento não
  permitir, a barra de LED (LM3914/15 ou módulo pronto) dá o mesmo movimento
  — e é o que o `AnalyserNode` do app imita.

> **Dica de execução:** imprima o painel do app em tamanho real (o app em 100 % de
> zoom tem as proporções da mesa), cole na chapa e fure pelos furos do papel. Depois
> tire o papel e use a chapa como gabarito. É assim que se faz painel sem CNC.

---

## 9. PCB no KiCad — do esquemático à placa

Ferramenta: **KiCad** (livre, roda em Linux/macOS/Windows). O PCB abre a porta do
Nível 3 e reduz o custo do Nível 1 (uma placa sai mais barata que 40 fios de
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
   nomes do app (`FILTER`, `SIRENE`, `SWEEP`, `SYNARE`, `SUB`…).

---

## 10. Firmware (Arduino Nano / RP2040) — opcional

O firmware **não faz áudio**. Ele faz o que o software faz de "inteligente":
medição, memória e comunicação. Se não quiser firmware, os Níveis 1 e 2 funcionam
sem ele.

| Função | Como |
|---|---|
| VU | Ler o nível na saída e acender a barra de LED / mover o ponteiro |
| Presets de sirene | Os **10 presets nomeados** do app viram 10 posições de memória; o Arduino grava os valores e aciona relés/digipots para "girar os knobs" |
| MIDI | Entrada MIDI (DIN ou USB) para controlar a mesa de um controlador externo |
| TAP TEMPO | Detectar o tempo e ajustar o delay (digipot no lugar do potenciômetro de TIME) |
| Auto-desligamento | Desligar a alimentação após X minutos sem sinal |

O firmware é onde o software e o hardware se encontram: o comportamento documentado
do app vira a especificação do firmware.

---

## 11. Fases

- [ ] **Fase 0 — bancada.** Comprar chips + potenciômetros do Nível 1. Nada soldado ainda.
- [ ] **Fase 1 — um canal.** Jack → pré → crossover → MASTER → saída. Ouvir.
- [ ] **Fase 2 — o caráter.** Sirene (CD4046) e depois Synare (LM13700 + ruído).
- [ ] **Fase 3 — o espaço.** Delay PT2399 com FB e filtro na realimentação.
- [ ] **Fase 4 — a mesa.** Kill/ONLY em todas as bandas, VU, segundo canal, X-FADER, caixa.
- [ ] **Fase 5 — a placa.** Esquemático e PCB no KiCad, mandar fabricar.
- [ ] **Fase 6 — a cabeça.** Firmware: VU, presets, MIDI, tap tempo.

Cada fase entrega um instrumento **tocável**. Se o projeto parar na Fase 3, ainda
existe uma mesa de dub com sirene e Synare — não uma caixa de peças.

---

## 12. Riscos e limites (honestidade antes de otimismo)

| Risco | Mitigação |
|---|---|
| **A conta cresce rápido quando a mesa fica completa** | Os níveis existem justamente pra isso: comece no Nível 1 (~R$ 400) e só suba com som tocando |
| **Ruído, chiado e oscilação** no áudio analógico | Desacoplar cada CI, terra estrela, trilhas curtas, testar bloco por bloco (seção 3) |
| **Comprar tudo de uma vez e não montar** | A Fase 0/1 é minúscula de propósito. Só comprar mais depois de ouvir a Fase 1 |
| **PT2399 curto demais** para o delay de 1,2 s | Dois PT2399 em série; ou aceitar ~600 ms e usar como *slap*, que é o som do dub raiz |
| **LM13700 ficou caro e rareou** (marcado como fim de vida pela TI) | Alternativas: VCO com CD4046, VCA com CD4066 + envelope RC; ou estocar uns chips. Perde qualidade, ganha disponibilidade |
| **Potenciômetro errado (A/B/curva)** mata a sensação | A coluna "componente físico" da seção 2 é obrigatória. O app define as faixas |
| **Polaridade e tensão erradas queimam chips** | Checar o datasheet duas vezes, 9 V antes de ±12 V, e usar soquete nos CIs |
| **36 knobs + 20 botões não cabem no primeiro painel** | O painel segue as fases: comece com 8 knobs (Nível 1) e cresça nos furos já planejados |

---

## 13. Próximo passo concreto

```
1. LISTA  — fechar a BOM do Nível 1 e conferir preço em UMA loja só        (1 noite)
2. PEDIR  — comprar APENAS a Fase 1: TL072, NE5532, CD4066, pots, 1 jack,
            chaves, resistores/caps, protoboard. Nada de PT2399/LM13700.   (1 dia)
3. OUVIR  — montar entrada -> crossover -> somador -> master.
            Ajustar níveis com trimpot. Fazer o KILL de uma banda.         (1 fim de semana)
4. GRAVAR — gravar um áudio de 30 s da Fase 1 e comparar de ouvido com
            o index.html com os mesmos ajustes. É o teste da spec.          (1 tarde)
5. SÓ ENTÃO comprar sirene, Synare e delay.
```

O passo 4 é o mais importante desta frente: **se a Fase 1 soar como o `index.html`
com os mesmos ajustes, a especificação está validada** e o resto é repetir o que
deu certo. Se não soar, descobrimos isso gastando ~R$ 400 em vez de ~R$ 2.500.

---

## Referências

- `rust/` — a outra frente: a versão executável nativa (Rust/Tauri) e os kernels DSP
- [Datasheet PT2399](https://www.google.com/search?q=PT2399+datasheet) — delay digital lofi
- [Datasheet CD4046](https://www.google.com/search?q=CD4046+datasheet) — PLL e VCO
- [Datasheet LM13700](https://www.google.com/search?q=LM13700+datasheet) — OTA: VCO e VCA
- [Datasheet TL072](https://www.google.com/search?q=TL072+datasheet) / [NE5532](https://www.google.com/search?q=NE5532+datasheet) — op-amps dos filtros
- [KiCad](https://www.kicad.org/) — esquemático e PCB, livre e multiplataforma
- [JLCPCB](https://jlcpcb.com/) / [PCBWay](https://www.pcbway.com/) — fabricação das placas
