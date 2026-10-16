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

  // 1. Mesa completa (RACK, todos os pedais visiveis)
  let p = await nova({ width: 1366, height: 1200 });
  await p.evaluate(() => { setModo('rack'); togglePedal('synare'); });
  await p.waitForTimeout(800);
  await p.screenshot({ path: path.join(OUT, 'mesa-completa.png'), fullPage: true });
  await p.locator('.toolbar').screenshot({ path: path.join(OUT, 'toolbar.png') });

  // 2. Mesa compacta (sirene + synare ocultos)
  await p.evaluate(() => { togglePedal('siren'); togglePedal('synare'); });
  await p.waitForTimeout(700);
  await p.screenshot({ path: path.join(OUT, 'mesa-compacta.png'), fullPage: true });
  await p.close();

  // 3. Modo RACK (pedais lado a lado)
  p = await nova({ width: 1366, height: 1400 });
  await p.evaluate(() => { setModo('rack'); togglePedal('siren'); togglePedal('synare'); });
  await p.waitForTimeout(900);
  await p.screenshot({ path: path.join(OUT, 'mesa-rack.png'), fullPage: true });
  await p.close();

  // 4. Modo DUB MIXER (compacto, tudo no gabinete)
  p = await nova({ width: 1366, height: 1400 });
  await p.evaluate(() => { setModo('dubmixer'); togglePedal('siren'); togglePedal('synare'); });
  await p.waitForTimeout(900);
  await p.screenshot({ path: path.join(OUT, 'mesa-dubmixer.png'), fullPage: true });
  await p.close();

  // 5. Sirene (rack 19") close
  p = await nova({ width: 1366, height: 1200 });
  await p.evaluate(() => { setModo('rack'); togglePedal('siren'); });
  await p.waitForTimeout(700);
  await p.locator('#modSiren').screenshot({ path: path.join(OUT, 'sirene.png') });
  await p.close();

  // 6. Pedal Synare close
  p = await nova({ width: 1366, height: 1200 });
  await p.evaluate(() => togglePedal('synare'));
  await p.waitForTimeout(700);
  await p.locator('#modSynare').screenshot({ path: path.join(OUT, 'pedal-synare.png') });
  await p.close();

  // 7. Master VU estereo
  p = await nova({ width: 1366, height: 900 });
  await p.locator('.master-tower').screenshot({ path: path.join(OUT, 'master-vu-estereo.png') });
  await p.close();

  // 8. Mobile
  p = await nova({ width: 390, height: 844 });
  await p.evaluate(() => togglePedal('synare'));
  await p.waitForTimeout(700);
  await p.screenshot({ path: path.join(OUT, 'mobile.png'), fullPage: true });
  await p.close();

  // 9. Temas (rasta, cosmic, neve, tube)
  for (const t of ['theme-rasta', 'theme-cosmic', 'theme-neve', 'theme-tube']) {
    p = await nova({ width: 1280, height: 1000 });
    await p.evaluate((th) => { setTheme(th); togglePedal('synare'); }, t);
    await p.waitForTimeout(700);
    await p.screenshot({ path: path.join(OUT, 'tema-' + t.replace('theme-','') + '.png'), fullPage: true });
    await p.close();
  }

  // 10. Toolbar com seletor de modo
  p = await nova({ width: 1366, height: 900 });
  await p.evaluate(() => setModo('rack'));
  await p.waitForTimeout(400);
  await p.locator('.toolbar').screenshot({ path: path.join(OUT, 'toolbar-modos.png') });
  await p.close();

  await browser.close();
  console.log('Screenshots gerados em ' + OUT);
  console.log(fs.readdirSync(OUT).filter(f => f.endsWith('.png')).sort().join('\n  '));
})();
