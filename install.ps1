# install.ps1
$src = "target\release\cerberus.exe"
$distDir = "dist"

Write-Host "--- Подготовка инсталляции версии 1.1.1 ---" -ForegroundColor Cyan

# 1. Проверяем, есть ли скомпилированный файл
if (!(Test-Path $src)) {
    Write-Error "Файл $src не найден! Сначала запусти 'cargo build --release'"
    exit 1
}

# 2. Создаем папку dist, если её нет
if (!(Test-Path $distDir)) {
    New-Item -ItemType Directory -Path $distDir | Out-Null
    Write-Host "Папка $distDir создана."
}

# 3. Копируем бинарник
Copy-Item $src -Destination "$distDir\cerberus.exe" -Force
Write-Host "Файл cerberus.exe успешно скопирован в $distDir" -ForegroundColor Green

Write-Host "Инсталляция завершена!" -ForegroundColor Yellow