#!/usr/bin/env bash
# ================================================================
#  DUB MIXER (macOS) - clique duas vezes aqui no Finder
#
#  Abre o mixer por um servidor local (http://localhost).
#
#  A janela do Terminal precisa ficar aberta enquanto voce usa o
#  mixer - e ela que mantem o servidor no ar.
# ================================================================

cd "$(dirname "$0")" || exit 1

echo
echo "  ============================================================"
echo "    DUB MIXER - servindo em http://127.0.0.1:8712"
echo "    Pode deixar esta janela aberta. Feche para encerrar."
echo "  ============================================================"
echo

if command -v bash >/dev/null 2>&1; then
    exec bash ./dub-mixer.sh
else
    exec sh ./dub-mixer.sh
fi
