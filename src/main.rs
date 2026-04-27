mod lib;
mod key_manager;

use dialoguer::{Select, Input, Confirm};
use console::style;
use std::{fs, process};
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    // Получаем ключ из системы
    let master_key = key_manager::get_or_create_key();

    loop {
        println!("\n{}", style("=== Cerberus Security System ===").bold().cyan());
        let options = vec![
            "Зашифровать файл", 
            "Расшифровать файл", 
            "Удалить все ключи (Uninstall)", 
            "Выход"
        ];
        
        let selection = Select::new().items(&options).interact()?;

        match selection {
            0 => handle_file("Encrypt", &master_key)?,
            1 => handle_file("Decrypt", &master_key)?,
            2 => uninstall()?,
            _ => break,
        }
    }
    Ok(())
}

fn handle_file(action: &str, key: &[u8]) -> anyhow::Result<()> {
    // Указываем тип String для Input, чтобы Rust не ругался
    let input: String = Input::<String>::new()
        .with_prompt("Введите имя файла или путь")
        .interact_text()?
        .trim()
        .to_string();
    
    let mut path = PathBuf::from(&input);

    if !path.exists() {
        if let Some(found_path) = find_file_globally(&input) {
            println!("{}", style(format!("Файл найден: {:?}", found_path)).green());
            path = found_path;
        } else {
            println!("{}", style("Ошибка: Файл не найден.").red());
            return Ok(());
        }
    }

    let data = fs::read(&path)?;
    // Вызываем функции из lib.rs
    let result = if action == "Encrypt" { 
        lib::encrypt(&data, key)? 
    } else { 
        lib::decrypt(&data, key)? 
    };

    fs::write(&path, result)?;
    println!("{}", style("Операция успешно завершена.").green());
    Ok(())
}

fn find_file_globally(filename: &str) -> Option<PathBuf> {
    println!("{}", style("Поиск файла...").yellow());
    for entry in WalkDir::new("C:\\").into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == filename {
            return Some(entry.path().to_path_buf());
        }
    }
    None
}

fn uninstall() -> anyhow::Result<()> {
    let confirm = Confirm::new().with_prompt("Удалить мастер-ключ из системы?").interact()?;
    if confirm {
        let entry = keyring::Entry::new("cerberus_app", "master_key")?;
        // Используем корректный метод для удаления в актуальной версии keyring
        entry.delete_credential()?; 
        println!("{}", style("Ключ удален. Файлы расшифровать невозможно.").bold().red());
        process::exit(0);
    }
    Ok(())
}