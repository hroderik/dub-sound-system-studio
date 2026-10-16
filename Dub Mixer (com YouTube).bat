@echo off
setlocal EnableExtensions
cd /d "%~dp0"

rem ================================================================
rem  Dub Mixer - abre o mixer por um servidor local (http://localhost)
rem  Isso e necessario para o YOUTUBE funcionar: o YouTube nao aceita
rem  tocar dentro de uma pagina aberta direto do disco (file://).
rem
rem  Este atalho: 1) sobe o servidor, 2) ESPERA ele responder,
rem  3) so entao abre o navegador na pagina certa.
rem ================================================================

set "PORT=8712"
set "PAGINA=index.html"
set "URL=http://127.0.0.1:%PORT%/%PAGINA%"

rem ---------- encontra o Python ----------
set "PYEXE="
if exist "%LOCALAPPDATA%\Programs\Python\Python312\python.exe" set "PYEXE=%LOCALAPPDATA%\Programs\Python\Python312\python.exe"
if not defined PYEXE if exist "%LOCALAPPDATA%\Programs\Python\Python311\python.exe" set "PYEXE=%LOCALAPPDATA%\Programs\Python\Python311\python.exe"
if not defined PYEXE if exist "%LOCALAPPDATA%\Programs\Python\Python310\python.exe" set "PYEXE=%LOCALAPPDATA%\Programs\Python\Python310\python.exe"
if not defined PYEXE if exist "%LOCALAPPDATA%\Programs\Python\Python39\python.exe" set "PYEXE=%LOCALAPPDATA%\Programs\Python\Python39\python.exe"
if not defined PYEXE (where py >nul 2>&1 && set "PYEXE=py")
if not defined PYEXE (where python >nul 2>&1 && set "PYEXE=python")

if not defined PYEXE goto :sempython

rem ---------- navegador ----------
set "FF=%ProgramFiles%\Mozilla Firefox\firefox.exe"
if not exist "%FF%" set "FF=%ProgramFiles(x86)%\Mozilla Firefox\firefox.exe"
if not exist "%FF%" set "FF="

set "TEMCURL=0"
where curl >nul 2>&1 && set "TEMCURL=1"

echo.
echo   Dub Mixer - preparando o servidor local na porta %PORT% ...
echo.

rem ---------- se o servidor ja esta no ar, apenas abre ----------
if "%TEMCURL%"=="1" (
    curl -s -o nul --max-time 2 "%URL%"
    if not errorlevel 1 goto :abrindo
)

rem ---------- sobe o servidor em uma janela minimizada ----------
start "Dub Mixer - servidor (nao feche)" /min "%PYEXE%" -m http.server %PORT% --bind 127.0.0.1

rem ---------- espera o servidor responder antes de abrir o navegador ----------
if "%TEMCURL%"=="0" goto :semcurl

set /a tentativas=0
:espera
curl -s -o nul --max-time 2 "%URL%"
if not errorlevel 1 goto :abrindo
set /a tentativas+=1
if %tentativas% geq 20 goto :falhou
timeout /t 1 /nobreak >nul
goto :espera

:semcurl
timeout /t 4 /nobreak >nul

:abrindo
if defined FF (
    start "" "%FF%" "%URL%"
) else (
    start "" "%URL%"
)
echo   PAGINA ABERTA EM:  %URL%
echo.
echo   Deixe a janelinha "Dub Mixer - servidor" minimizada enquanto usa o
echo   mixer - e ela que faz o YOUTUBE tocar. Para encerrar, feche-a.
echo.
timeout /t 10 >nul
exit /b 0

:falhou
echo.
echo   Nao consegui iniciar o servidor local na porta %PORT%.
echo.
echo   Teste manualmente: abra o PowerShell e rode
echo       "%PYEXE%" -m http.server %PORT% --bind 127.0.0.1
echo   Depois abra no navegador:  %URL%
echo.
echo   Se aparecer algum erro, copie a mensagem e me envie.
echo.
pause
exit /b 1

:sempython
echo.
echo   Python nao encontrado neste PC.
echo   Instale em https://www.python.org/downloads/
echo   IMPORTANTE: marque a opcao "Add python.exe to PATH" na instalacao.
echo.
pause
exit /b 1
