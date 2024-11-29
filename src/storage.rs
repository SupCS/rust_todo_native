use crate::tasks::Task;
use serde_json;
use std::fs::{self, File};
use std::io::Write;

// Зберегти завдання у файл
pub fn save_tasks_to_file(tasks: &Vec<Task>, file_name: &str) {
    let file = File::create(file_name).expect("Не вдалося створити файл");
    let json = serde_json::to_string(tasks).expect("Не вдалося серіалізувати завдання");
    writeln!(&file, "{}", json).expect("Не вдалося записати у файл");
    println!("Список завдань збережено у файл: {}", file_name);
}

// Завантажити завдання з файлу
pub fn load_tasks_from_file(file_name: &str) -> Vec<Task> {
    let content = fs::read_to_string(file_name).expect("Не вдалося прочитати файл");
    serde_json::from_str(&content).expect("Не вдалося десеріалізувати завдання")
}
