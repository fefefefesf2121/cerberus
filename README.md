## Установка

1. Установите Rust: [rustup.rs](https://rustup.rs/)
2. Склонируйте репозиторий:
   ```bash
   git clone [https://github.com/fefefefesf2121/cerberus.git](https://github.com/fefefefesf2121/cerberus.git)
   cd cerberus
   Windows (PowerShell):
   $env:CERBERUS_KEY="твой-очень-длинный-секретный-ключ-32-символа"
./target/release/cerberus.exe encrypt --file secret.txt
Linux/macOS:
export CERBERUS_KEY="твой-очень-длинный-секретный-ключ-32-символа"
./target/release/cerberus decrypt --file secret.txt.enc