#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Servidor local do Dub Mixer (com cabecalho anti-cache).

Por que este arquivo existe:
    O comando "python -m http.server" NAO manda cabecalho de cache. Assim o
    navegador guarda uma copia da pagina e pode continuar mostrando uma versao
    VELHA do mixer mesmo depois de voce atualizar o arquivo (foi exatamente o
    que aconteceu: o atalho abria uma versao antiga).

    Aqui todo arquivo sai com "Cache-Control: no-store", entao o navegador
    sempre pega a versao que esta no disco agora.

Uso:
    python3 serve.py [porta]      # padrao: 8712
"""

import os
import sys
import http.server
import socketserver
import webbrowser

PORTA = int(sys.argv[1]) if len(sys.argv) > 1 else 8712
PASTA = os.path.dirname(os.path.abspath(__file__))


class SemCache(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=PASTA, **kwargs)

    def end_headers(self):
        self.send_header("Cache-Control", "no-store, no-cache, must-revalidate, max-age=0")
        self.send_header("Pragma", "no-cache")
        self.send_header("Expires", "0")
        super().end_headers()

    def log_message(self, fmt, *args):
        pass  # silencioso: nao enche a janela do usuario


class ServidorReutilizavel(socketserver.ThreadingTCPServer):
    allow_reuse_address = True
    daemon_threads = True


def main():
    try:
        servidor = ServidorReutilizavel(("127.0.0.1", PORTA), SemCache)
    except OSError as e:
        print()
        print("  Nao consegui usar a porta %d (%s)." % (PORTA, e))
        print("  Provavelmente ja existe um servidor rodando nessa porta.")
        print("  Feche a janelinha antiga do servidor e rode de novo.")
        print()
        return 1

    print()
    print("  ============================================================")
    print("    DUB MIXER - servindo a pasta:")
    print("    %s" % PASTA)
    print("    (sem cache: o navegador sempre abre a versao mais nova)")
    print("  ============================================================")
    print()
    print("    http://127.0.0.1:%d/" % PORTA)
    print()
    print("    Pode deixar esta janela aberta. Feche-a para encerrar.")
    print()
    try:
        servidor.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        servidor.server_close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
