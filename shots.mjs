/* Gera os prints do README em docs/img usando o Playwright.
   Passos:
     1) npm install                       (só playwright-core, navegador não é baixado)
     2) servir o app em http://127.0.0.1:8712  (na pasta do repo, ex.: python3 -m http.server 8712)
     3) npm run shots
   Usa o Edge ou o Chrome já instalados no sistema. */
import { chromium } from 'playwright-core';
import fs from 'fs';
import path from 'path';

const OUT = './docs/img';
const APP = 'http://127.0.0.1:8712/index.html';

const espera = ms => new Promise(r => setTimeout(r, ms));
const problemas = [];   // erros de console/página (ex.: violação de CSP) — mostrados no fim

function monitorar(pagina, cena) {
  pagina.on('console', m => { if (m.type() === 'error') problemas.push(`[${cena}] console: ${m.text()}`); });
  pagina.on('pageerror', e => problemas.push(`[${cena}] pageerror: ${e.message}`));
}

(async () => {
  fs.mkdirSync(OUT, { recursive: true });

  // Navegador: Edge ou Chrome já instalados (sem download)
  let browser = null;
  for (const canal of ['msedge', 'chrome']) {
    try {
      browser = await chromium.launch({ channel: canal, args: ['--no-sandbox'] });
      console.log('navegador: ' + canal);
      break;
    } catch (e) {
      console.log('canal "' + canal + '" indisponível');
    }
  }
  if (!browser) { console.error('Nem Edge nem Chrome encontrados neste PC.'); process.exit(1); }

  async function nova(vp, cena) {
    const p = await browser.newPage({ viewport: vp, deviceScaleFactor: 1 });
    monitorar(p, cena);
    await p.goto(APP, { waitUntil: 'load' });
    await p.evaluate(() => document.fonts.ready);
    await espera(1200);
    return p;
  }

  // 1. Mesa completa (todos os pedais visíveis)
  let p = await nova({ width: 1366, height: 1200 }, 'mesa-completa');
  await p.evaluate(() => togglePedal('synare'));
  await espera(800);
  await p.screenshot({ path: path.join(OUT, 'mesa-completa.png'), fullPage: true });
  await p.locator('.toolbar').screenshot({ path: path.join(OUT, 'toolbar.png') });
  await p.close();

  // 2. Mesa compacta (só o gabinete, sem os módulos externos)
  p = await nova({ width: 1366, height: 1000 }, 'mesa-compacta');
  await p.evaluate(() => togglePedal('siren'));
  await espera(700);
  await p.screenshot({ path: path.join(OUT, 'mesa-compacta.png'), fullPage: true });
  await p.close();

  // 3. MODO EDIÇÃO (mover os módulos e o próprio DUB MIXER na ordem que quiser)
  p = await nova({ width: 1366, height: 1400 }, 'modo-edicao');
  await p.evaluate(() => { togglePedal('synare'); toggleEdicao(); });
  await espera(900);
  await p.screenshot({ path: path.join(OUT, 'modo-edicao.png'), fullPage: true });
  await p.close();

  // 4. Sirene (rack 19") close
  p = await nova({ width: 1366, height: 1200 }, 'sirene');
  await espera(700);
  await p.locator('#modSiren').screenshot({ path: path.join(OUT, 'sirene.png') });
  await p.close();

  // 5. Pedal Synare close
  p = await nova({ width: 1366, height: 1200 }, 'pedal-synare');
  await p.evaluate(() => togglePedal('synare'));
  await espera(700);
  await p.locator('#modSynare').screenshot({ path: path.join(OUT, 'pedal-synare.png') });
  await p.close();

  // 6. Master VU estéreo (tema com VU analógico: mostradores retangulares laranja)
  p = await nova({ width: 1366, height: 900 }, 'master-vu');
  await p.evaluate(() => setTheme('theme-neve'));
  await espera(600);
  await p.locator('.master-tower').screenshot({ path: path.join(OUT, 'master-vu-estereo.png') });
  await p.close();

  // 7. EQ & FILTERS (LOWPASS + HIGHPASS)
  p = await nova({ width: 1366, height: 1100 }, 'eq-filters');
  await espera(600);
  await p.locator('.channel-strip').first().screenshot({ path: path.join(OUT, 'eq-cutoff-pan.png') });
  await p.close();

  // 8. Mobile
  p = await nova({ width: 390, height: 844 }, 'mobile');
  await p.evaluate(() => togglePedal('synare'));
  await espera(700);
  await p.screenshot({ path: path.join(OUT, 'mobile.png'), fullPage: true });
  await p.close();

  // 9. Temas (rasta, cosmic, neve, tube)
  for (const t of ['theme-rasta', 'theme-cosmic', 'theme-neve', 'theme-tube']) {
    p = await nova({ width: 1280, height: 1000 }, 'tema-' + t);
    await p.evaluate((th) => { setTheme(th); togglePedal('synare'); }, t);
    await espera(700);
    await p.screenshot({ path: path.join(OUT, 'tema-' + t.replace('theme-', '') + '.png'), fullPage: true });
    await p.close();
  }

  await browser.close();
  console.log('Prints gerados em ' + OUT + ':');
  console.log('  ' + fs.readdirSync(OUT).filter(f => f.endsWith('.png')).sort().join('\n  '));
  if (problemas.length) {
    console.log('\nAVISOS DO NAVEGADOR (' + problemas.length + '):');
    console.log(problemas.slice(0, 30).join('\n'));
    process.exitCode = 2;
  } else {
    console.log('\nNenhum erro de console/página — CSP e recursos OK.');
  }
})();
