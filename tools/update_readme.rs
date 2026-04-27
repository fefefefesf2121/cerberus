use serde::Deserialize;
use std::fs;
use std::io::Write;

// Структура данных из "облака" (твой плейсхолдер)
#[derive(Deserialize, Debug)]
struct CloudData {
    version: String,
    status: String,
    downloads: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Получение данных из облака...");

    // Твой плейсхолдер API (здесь имитируем получение данных)
    let url = "https://jsonplaceholder.typicode.com/posts/1"; // Замени на свое API
    
    // В реальности ты бы делал: let data: CloudData = reqwest::get(url).await?.json().await?;
    // Пока создадим заглушку данных:
    let data = CloudData {
        version: "0.1.0".to_string(),
        status: "Stable".to_string(),
        downloads: 1337,
    };

    // Формируем контент README
    let readme_content = format!(
        "# Cerberus\n\n\
        Это проект для безопасного шифрования.\n\n\
        ## Статистика проекта (автообновляемая)\n\
        * **Текущая версия:** {}\n\
        * **Статус:** {}\n\
        * **Загрузок:** {}\n\n\
        ## Установка\n\
        `cargo build --release`",
        data.version, data.status, data.downloads
    );

    // Записываем в файл
    let mut file = fs::File::create("README.md")?;
    file.write_all(readme_content.as_bytes())?;

    println!("README.md успешно обновлен!");
    Ok(())
}