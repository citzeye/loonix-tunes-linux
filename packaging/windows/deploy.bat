@echo off
set DEPLOY_DIR=release_windows
mkdir %DEPLOY_DIR%

:: 1. Copy Binary
copy ..\..\target\release\loonix-tunes.exe %DEPLOY_DIR%\

:: 2. Tarik Qt Dependencies (Wajib)
:: Ganti path ini sesuai lokasi Qt lo
windeployqt --qmldir ..\..\qml %DEPLOY_DIR%\loonix-tunes.exe

:: 3. Copy FFmpeg & DSP DLLs manual
:: Lo harus taruh file .dll (rubberband, fftw3, dll) di folder ini
copy bin\*.dll %DEPLOY_DIR%\

echo Windows Deployment Ready in %DEPLOY_DIR%