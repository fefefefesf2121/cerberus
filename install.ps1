# Создаем папку для программы
$installDir = "C:\Program Files\Cerberus"
New-Item -ItemType Directory -Force -Path $installDir

# Копируем скомпилированный файл
Copy-Item ".\target\release\cerberus.exe" -Destination "$installDir\cerberus.exe"

# Добавляем путь в переменную окружения PATH (чтобы команда cerberus работала везде)
$oldPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
if (!$oldPath.Contains($installDir)) {
    $newPath = $oldPath + ";" + $installDir
    [System.Environment]::SetEnvironmentVariable("Path", $newPath, "User")
}

Write-Host "Установка завершена! Перезапусти терминал и введи 'cerberus'" -ForegroundColor Green