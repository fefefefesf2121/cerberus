use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::process;

#[derive(Parser)]
#[command(name = "cerberus")]
#[command(version = "1.1.0")]
#[command(about = "Безопасный шифратор файлов", long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt { file: String },
    Decrypt { file: String },
}

fn main() {
    // Graceful exit: проверка ключа без паники
    let key = std::env::var("CERBERUS_KEY").unwrap_or_else(|_| {
        eprintln!("Ошибка: Переменная окружения CERBERUS_KEY не установлена!");
        process::exit(1);
    });

    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt { file } => {
            println!("🔒 Шифрую файл: {}", file);
            run_with_progress("Шифрование", 100); // Здесь будет твоя логика шифрования
            println!("Готово!");
        }
        Commands::Decrypt { file } => {
            println!("🔓 Расшифровываю файл: {}", file);
            run_with_progress("Расшифровка", 100);
            println!("Готово!");
        }
    }
}

fn run_with_progress(message: &str, total: u64) {
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({msg})")
        .unwrap()
        .progress_chars("#>-"));

    for _ in 0..total {
        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(10)); // Имитация работы
    }
    pb.finish_with_message(format!("{} завершено", message));
}