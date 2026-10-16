/* tape_delay.c — kernel de delay de fita (portável, sem alocação na thread de áudio)
 *
 * Compilar para WASM (quando emscripten disponível):
 *   emcc tape_delay.c -O3 -s WASM=1 -s EXPORTED_FUNCTIONS="['_tape_delay_init','_tape_delay_process']" -o tape_delay.wasm
 *
 * Uso: o host (AudioWorklet) aloca o buffer de delay uma vez e chama
 * tape_delay_process() a cada bloco de 128 amostras.
 */

#include <math.h>

#define MAX_DELAY_SAMPLES 192000  /* ~4s a 48kHz */

typedef struct {
    float buffer[MAX_DELAY_SAMPLES];
    int   write_pos;
    int   delay_samples;
    float feedback;
    float wet;
    float filter_state; /* passa-baixa simples no caminho do delay */
    int   sample_rate;
} TapeDelay;

void tape_delay_init(TapeDelay *d, int sample_rate) {
    d->sample_rate = sample_rate;
    d->write_pos = 0;
    d->delay_samples = (int)(0.375f * sample_rate);
    d->feedback = 0.4f;
    d->wet = 0.0f;
    d->filter_state = 0.0f;
    for (int i = 0; i < MAX_DELAY_SAMPLES; i++) d->buffer[i] = 0.0f;
}

void tape_delay_set_params(TapeDelay *d, float time_sec, float feedback, float wet) {
    int ds = (int)(time_sec * d->sample_rate);
    if (ds < 1) ds = 1;
    if (ds >= MAX_DELAY_SAMPLES) ds = MAX_DELAY_SAMPLES - 1;
    d->delay_samples = ds;
    if (feedback < 0.0f) feedback = 0.0f;
    if (feedback > 0.95f) feedback = 0.95f;
    d->feedback = feedback;
    if (wet < 0.0f) wet = 0.0f;
    if (wet > 1.0f) wet = 1.0f;
    d->wet = wet;
}

/* Processa um bloco. in/out podem ser o mesmo buffer (in-place) se dry for 0,
   mas aqui mantemos dry + wet separados para o host mixar. */
void tape_delay_process(TapeDelay *d, const float *in, float *out_dry, float *out_wet, int n) {
    for (int i = 0; i < n; i++) {
        int read_pos = d->write_pos - d->delay_samples;
        if (read_pos < 0) read_pos += MAX_DELAY_SAMPLES;
        float delayed = d->buffer[read_pos];

        /* filtro passa-baixa simples (eco escuro de fita) */
        d->filter_state += 0.15f * (delayed - d->filter_state);
        delayed = d->filter_state;

        /* realimentacao */
        d->buffer[d->write_pos] = in[i] + delayed * d->feedback;
        d->write_pos++;
        if (d->write_pos >= MAX_DELAY_SAMPLES) d->write_pos = 0;

        out_dry[i] = in[i];
        out_wet[i] = delayed * d->wet;
    }
}
