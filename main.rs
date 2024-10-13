use std::process::Command;
use std::io::{self, Write};

fn main() {
    // Запрашиваем имя ветки у пользователя
    let branch_name = input("Введите имя для новой ветки: ");

    // Выполняем команду git branch branchname
    run_command("git", &["branch", &branch_name]);

    // Переключаемся на новую ветку
    run_command("git", &["checkout", &branch_name]);

    // Запрашиваем у пользователя файл/папку для добавления
    let file_to_add = input("Введите имя файла или папки для добавления: ");

    // Выполняем команду git add filename
    run_command("git", &["add", &file_to_add]);

    // Запрашиваем сообщение для коммита
    let commit_message = input("Введите сообщение для коммита: ");

    // Выполняем команду git commit -m "message"
    run_command("git", &["commit", "-m", &commit_message]);

    // Выполняем команду git push origin branchname
    run_command("git", &["push", "origin", &branch_name]);

    println!("Готово!");
}

// Функция для выполнения системной команды
fn run_command(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args)
        .status()
        .expect("Не удалось выполнить команду");

    if !status.success() {
        eprintln!("Ошибка при выполнении команды: {:?}", args);
    }
}

// Функция для ввода данных от пользователя
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    input.trim().to_string() // Убираем лишние пробелы и символы новой строки
}
