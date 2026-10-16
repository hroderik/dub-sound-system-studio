#!/usr/bin/env bash
# ================================================================
#  Dub Mixer - abre o mixer por um servidor local (http://localhost)
#  (Util para evitar cache e servir para o celular na mesma Wi-Fi.)
#
#  Funciona no Linux e no macOS. Este script:
#    1) sobe o servidor, 2) ESPERA ele responder,
#    3) so entao abre o navegador na pagina certa.
#
#  Uso:   ./dub-mixer.sh
# ================================================================

set -u

PORT=8712
PAGINA=index.html
URL="http://127.0.0.1:${PORT}/${PAGINA}"
# O "?v=..." so serve para furar cache de navegador.
URL_ABRIR="${URL}?v=$(date +%s 2>/dev/null || echo 0)"

# --- vai para a pasta do script (funciona tambem com ./caminho/dub-mixer.sh) ---
cd "$(dirname "$0")" || exit 1

echo
echo "  Dub Mixer - preparando o servidor local na porta ${PORT} ..."
echo

# ---------- encontra o Python ----------
PYEXE=""
for c in python3 python; do
    if command -v "$c" >/dev/null 2>&1; then PYEXE="$c"; break; fi
done

if [ -z "$PYEXE" ]; then
    echo "  Python nao encontrado neste sistema."
    echo "  Debian/Ubuntu:  sudo apt install python3"
    echo "  Fedora:         sudo dnf install python3"
    echo "  macOS:          brew install python3"
    echo "  Depois rode de novo:  ./dub-mixer.sh"
    echo
    exit 1
fi

# ---------- ja tem servidor no ar? ----------
if command -v curl >/dev/null 2>&1 && curl -s -o /dev/null --max-time 2 "$URL"; then
    ABRE_SOZINHO=1
else
    ABRE_SOZINHO=0
fi

# ---------- sobe o servidor em segundo plano ----------
if [ "$ABRE_SOZINHO" = "0" ]; then
    echo "  Iniciando servidor com: $PYEXE -m http.server $PORT --bind 127.0.0.1"
    "$PYEXE" -m http.server "$PORT" --bind 127.0.0.1 >/dev/null 2>&1 &
    SERVIDOR_PID=$!

    # ---------- espera o servidor responder antes de abrir o navegador ----------
    if command -v curl >/dev/null 2>&1; then
        tentativas=0
        until curl -s -o /dev/null --max-time 2 "$URL"; do
            tentativas=$((tentativas + 1))
            if [ "$tentativas" -ge 20 ]; then
                echo
                echo "  Nao consegui iniciar o servidor local na porta ${PORT}."
                echo "  Tente manualmente:  $PYEXE -m http.server $PORT --bind 127.0.0.1"
                echo "  Depois abra no navegador:  $URL"
                echo
                exit 1
            fi
            sleep 1
        done
    else
        sleep 4   # sem curl: espera fixa
    fi

    # mata o servidor quando o script sai (ou quando a janela e fechada)
    trap 'kill "$SERVIDOR_PID" 2>/dev/null' EXIT INT TERM
fi

# ---------- navegador ----------
if [ "$(uname -s)" = "Darwin" ]; then
    # macOS
    if command -v open >/dev/null 2>&1; then open "$URL_ABRIR"; fi
else
    # Linux
    for nav in xdg-open firefox google-chrome chromium-browser chromium sensible-browser; do
        if command -v "$nav" >/dev/null 2>&1; then "$nav" "$URL_ABRIR" >/dev/null 2>&1 & break; fi
    done
fi

echo "  PAGINA ABERTA EM:  $URL"
echo
echo "  Enquanto este terminal estiver aberto, o servidor continua no ar - e"
echo "  Para encerrar, feche a janela (Ctrl+C)."
echo
echo "  Celular na mesma Wi-Fi? Use:  http://$(hostname -I 2>/dev/null | awk '{print $1}'):${PORT}/${PAGINA}"
echo

# mantem o terminal vivo enquanto o servidor roda
if [ "$ABRE_SOZINHO" = "0" ] && command -v curl >/dev/null 2>&1; then
    while kill -0 "$SERVIDOR_PID" 2>/dev/null; do sleep 1; done
fi
