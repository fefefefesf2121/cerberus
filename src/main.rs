use clap::{Parser, Subcommand};
use cerberus::Cerberus;
use std::{env, fs};

#[derive(Parser)]
#[command(name = "Cerberus", version = "1.0", about = "Безопасный шифратор файлов")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Зашифровать файл
    Encrypt { file: String },
    /// Расшифровать файл
    Decrypt { file: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Сначала парсим аргументы (это безопасно, если есть --help)
    let cli = Cli::parse();

    // 2. Только если пользователь выбрал команду, пробуем получить ключ
    match &cli.command {
        Commands::Encrypt { file } | Commands::Decrypt { file } => {
            let key = env::var("CERBERUS_KEY")
                .expect("ОШИБКА: Переменная CERBERUS_KEY не установлена!");

            if key.len() < 32 {
                panic!("Ключ CERBERUS_KEY должен быть не менее 32 символов!");
            }

            let c = Cerberus::new(key.as_bytes());

            // 3. Выполняем логику
            match &cli.command {
                Commands::Encrypt { .. } => {
                    let data = fs::read(file)?;
                    let encrypted = c.encrypt(&data)?;
                    fs::write(format!("{}.enc", file), encrypted)?;
                    println!("Файл {} зашифрован.", file);
                }
                Commands::Decrypt { .. } => {
                    let data = fs::read(file)?;
                    let decrypted = c.decrypt(&data)?;
                    fs::write(format!("decrypted_{}", file), decrypted)?;
                    println!("Файл {} расшифрован.", file);
                }
            }
        }
    }
    Ok(())
}