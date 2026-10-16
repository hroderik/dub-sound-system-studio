/* tape_delay_processor.js — AudioWorkletProcessor (mirror do kernel C)
 *
 * Registra o processador "tape-delay" que o host usa via AudioWorkletNode.
 * Mesma lógica do tape_delay.c, em JS, para provar a rota sem precisar
 * compilar C→WASM. Quando o .wasm estiver disponível, o host pode trocar
 * o processamento para o kernel WASM (mesmo contrato de parâmetros).
 */

class TapeDelayProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.MAX = 192000;
        this.buf = new Float32Array(this.MAX);
        this.writePos = 0;
        this.delaySamples = Math.floor(0.375 * sampleRate);
        this.feedback = 0.4;
        this.wet = 0.0;
        this.filterState = 0.0;
        this.port.onmessage = (e) => {
            const { time, fb, wet } = e.data || {};
            if (time !== undefined) {
                let ds = Math.floor(time * sampleRate);
                if (ds < 1) ds = 1;
                if (ds >= this.MAX) ds = this.MAX - 1;
                this.delaySamples = ds;
            }
            if (fb !== undefined) this.feedback = Math.max(0, Math.min(0.95, fb));
            if (wet !== undefined) this.wet = Math.max(0, Math.min(1, wet));
        };
    }
    process(inputs, outputs) {
        const input = inputs[0];
        const output = outputs[0];
        if (!input || !input[0] || !output || !output[0]) return true;
        const inCh = input[0];
        const outCh = output[0];
        const n = inCh.length;
        for (let i = 0; i < n; i++) {
            let readPos = this.writePos - this.delaySamples;
            if (readPos < 0) readPos += this.MAX;
            let delayed = this.buf[readPos];
            // filtro passa-baixa (eco escuro de fita)
            this.filterState += 0.15 * (delayed - this.filterState);
            delayed = this.filterState;
            this.buf[this.writePos] = inCh[i] + delayed * this.feedback;
            this.writePos++;
            if (this.writePos >= this.MAX) this.writePos = 0;
            // dry + wet
            outCh[i] = inCh[i] + delayed * this.wet;
        }
        // copia para outros canais (estereo)
        for (let c = 1; c < output.length; c++) {
            for (let i = 0; i < n; i++) output[c][i] = outCh[i];
        }
        return true;
    }
}
registerProcessor('tape-delay', TapeDelayProcessor);
