mod storage;
mod tasks;

use std::io;
use storage::{load_tasks_from_file, save_tasks_to_file};
use tasks::{add_task, edit_task, mark_done, remove_task, Task};

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Помилка вводу");
    input.trim().to_string()
}

fn parse_index(input: &str, max_index: usize) -> Option<usize> {
    match input.parse::<isize>() {
        Ok(num) if num > 0 && (num as usize) <= max_index => Some((num as usize) - 1),
        _ => None, // Невірне введення або індекс за межами допустимих
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\nМеню:");
        println!("1. Додати завдання");
        println!("2. Переглянути список завдань");
        println!("3. Редагувати завдання");
        println!("4. Видалити завдання");
        println!("5. Позначити завдання як виконане");
        println!("6. Зберегти завдання у файл");
        println!("7. Завантажити завдання з файлу");
        println!("8. Вийти");

        let choice = get_user_input("Оберіть пункт меню:");

        match choice.as_str() {
            "1" => {
                let name = get_user_input("Введіть назву завдання:");
                add_task(&mut tasks, &name);
            }
            "2" => {
                println!("\nСписок завдань:");
                for (i, task) in tasks.iter().enumerate() {
                    println!(
                        "{}: {} [{}]",
                        i + 1,
                        task.name,
                        if task.done { "✓" } else { " " }
                    );
                }
            }
            "3" => {
                let index_input = get_user_input("Введіть номер завдання для редагування:");
                if let Some(index) = parse_index(&index_input, tasks.len()) {
                    let new_name = get_user_input("Введіть нову назву завдання:");
                    edit_task(&mut tasks, index, &new_name);
                } else {
                    println!("Помилка: недійсний номер завдання.");
                }
            }
            "4" => {
                let index_input = get_user_input("Введіть номер завдання для видалення:");
                if let Some(index) = parse_index(&index_input, tasks.len()) {
                    remove_task(&mut tasks, index);
                } else {
                    println!("Помилка: недійсний номер завдання.");
                }
            }
            "5" => {
                let index_input =
                    get_user_input("Введіть номер завдання для позначення як виконане:");
                if let Some(index) = parse_index(&index_input, tasks.len()) {
                    mark_done(&mut tasks, index);
                } else {
                    println!("Помилка: недійсний номер завдання.");
                }
            }
            "6" => {
                save_tasks_to_file(&tasks, "tasks.json");
            }
            "7" => {
                tasks = load_tasks_from_file("tasks.json");
                println!("Завдання успішно завантажено з файлу.");
            }
            "8" => {
                println!("Вихід...");
                break;
            }
            _ => println!("Невірний вибір, спробуйте ще раз."),
        }
    }
}
