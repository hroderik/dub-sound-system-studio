const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

const OUT = './docs/img';
const APP = 'http://127.0.0.1:8712/index.html';

(async () => {
  const browser = await chromium.launch({ args: ['--no-sandbox'] });
  fs.mkdirSync(OUT, { recursive: true });

  async function nova(vp) {
    const p = await browser.newPage({ viewport: vp, deviceScaleFactor: 1 });
    await p.goto(APP, { waitUntil: 'load' });
    await p.waitForTimeout(1200);
    return p;
  }

  // 1. Mesa completa (todos os pedais visíveis)
  let p = await nova({ width: 1366, height: 1200 });
  await p.evaluate(() => togglePedal('synare'));
  await p.waitForTimeout(800);
  await p.screenshot({ path: path.join(OUT, 'mesa-completa.png'), fullPage: true });
  await p.locator('.toolbar').screenshot({ path: path.join(OUT, 'toolbar.png') });
  await p.close();

  // 2. Mesa compacta (só o gabinete, sem os módulos externos)
  p = await nova({ width: 1366, height: 1000 });
  await p.evaluate(() => togglePedal('siren'));
  await p.waitForTimeout(700);
  await p.screenshot({ path: path.join(OUT, 'mesa-compacta.png'), fullPage: true });
  await p.close();

  // 3. Visual PREAMP (painel preto do pré-amp clássico)
  p = await nova({ width: 1366, height: 1400 });
  await p.evaluate(() => { togglePreamp(); togglePedal('synare'); });
  await p.waitForTimeout(900);
  await p.screenshot({ path: path.join(OUT, 'preamp.png'), fullPage: true });
  await p.close();

  // 4. MODO EDIÇÃO (mover os módulos na ordem que quiser)
  p = await nova({ width: 1366, height: 1400 });
  await p.evaluate(() => { togglePedal('synare'); toggleEdicao(); });
  await p.waitForTimeout(900);
  await p.screenshot({ path: path.join(OUT, 'modo-edicao.png'), fullPage: true });
  await p.close();

  // 5. Sirene (rack 19") close
  p = await nova({ width: 1366, height: 1200 });
  await p.waitForTimeout(700);
  await p.locator('#modSiren').screenshot({ path: path.join(OUT, 'sirene.png') });
  await p.close();

  // 6. Pedal Synare close
  p = await nova({ width: 1366, height: 1200 });
  await p.evaluate(() => togglePedal('synare'));
  await p.waitForTimeout(700);
  await p.locator('#modSynare').screenshot({ path: path.join(OUT, 'pedal-synare.png') });
  await p.close();

  // 7. Master VU estéreo (tema com VU analógico: mostradores retangulares laranja)
  p = await nova({ width: 1366, height: 900 });
  await p.evaluate(() => setTheme('theme-neve'));
  await p.waitForTimeout(600);
  await p.locator('.master-tower').screenshot({ path: path.join(OUT, 'master-vu-estereo.png') });
  await p.close();

  // 8. EQ & FILTERS (LOWPASS + HIGHPASS)
  p = await nova({ width: 1366, height: 1100 });
  await p.waitForTimeout(600);
  await p.locator('.channel-strip').first().screenshot({ path: path.join(OUT, 'eq-cutoff-pan.png') });
  await p.close();

  // 9. Mobile
  p = await nova({ width: 390, height: 844 });
  await p.evaluate(() => togglePedal('synare'));
  await p.waitForTimeout(700);
  await p.screenshot({ path: path.join(OUT, 'mobile.png'), fullPage: true });
  await p.close();

  // 10. Temas (rasta, cosmic, neve, tube)
  for (const t of ['theme-rasta', 'theme-cosmic', 'theme-neve', 'theme-tube']) {
    p = await nova({ width: 1280, height: 1000 });
    await p.evaluate((th) => { setTheme(th); togglePedal('synare'); }, t);
    await p.waitForTimeout(700);
    await p.screenshot({ path: path.join(OUT, 'tema-' + t.replace('theme-', '') + '.png'), fullPage: true });
    await p.close();
  }

  await browser.close();
  console.log('Screenshots gerados em ' + OUT);
  console.log(fs.readdirSync(OUT).filter(f => f.endsWith('.png')).sort().join('\n  '));
})();
