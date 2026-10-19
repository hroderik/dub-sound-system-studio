/* DUB MIXER Sound System — service worker (PWA / uso offline)
   Regras de segurança:
   - só cacheia GET do MESMO domínio;
   - navegação tenta a rede e, se falhar, cai no index.html do cache;
   - nunca intercepta POST nem outros domínios. */
const CACHE = 'dub-mixer-v3';
const ASSETS = ['./', './index.html', './manifest.json', './icon-192.png', './icon-512.png'];

self.addEventListener('install', e => {
  e.waitUntil(caches.open(CACHE).then(c => c.addAll(ASSETS)).catch(() => {}));
  self.skipWaiting();
});
self.addEventListener('activate', e => {
  e.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(k => k !== CACHE).map(k => caches.delete(k)))));
  self.clients.claim();
});
self.addEventListener('fetch', e => {
  const req = e.request;
  if (req.method !== 'GET') return;                 /* nada de POST/PUT */
  const url = new URL(req.url);
  if (url.origin !== self.location.origin) return;  /* nada de outros domínios */
  if (req.mode === 'navigate') {                    /* a página em si: rede primeiro */
    e.respondWith(fetch(req).catch(() => caches.match('./index.html')));
    return;
  }
  e.respondWith(
    caches.match(req).then(hit => hit || fetch(req).then(res => {
      if (res && res.ok && res.type === 'basic') {
        const copy = res.clone();
        caches.open(CACHE).then(c => c.put(req, copy));
      }
      return res;
    }).catch(() => caches.match('./index.html')))
  );
});
