# Cerberus 🛡️

Мощный и минималистичный инструмент для шифрования файлов на Rust. Использует современный алгоритм AES-256-GCM для обеспечения максимальной безопасности.

## Установка

1. Установите Rust: [rustup.rs](https://rustup.rs/)
2. Склонируйте репозиторий:
   ```bash
   git clone [https://github.com/ВАШ_НИК/cerberus.git](https://github.com/ВАШ_НИК/cerberus.git)
   cd cerberus
   Windows (PowerShell):
   $env:CERBERUS_KEY="твой-очень-длинный-секретный-ключ-32-символа"
./target/release/cerberus.exe encrypt --file secret.txt
Linux/macOS:
export CERBERUS_KEY="твой-очень-длинный-секретный-ключ-32-символа"
./target/release/cerberus decrypt --file secret.txt.enc