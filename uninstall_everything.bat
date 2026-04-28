@echo off
echo WARNING: This will delete everything in this folder (including the .exe and keys)!
pause
:: Удаляем все файлы, кроме самого этого скрипта
del /f /q *.enc
del /f /q *.dec
del /f /q key.bin
del /f /q cerberus.exe
:: Удаляем папку target, если она есть
rd /s /q target
echo [!] Application uninstalled. Please delete this folder manually now.
pause