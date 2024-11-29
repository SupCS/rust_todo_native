use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)] // трейти
pub struct Task {
    pub name: String,
    pub done: bool,
}

// Додати нове завдання
pub fn add_task(tasks: &mut Vec<Task>, name: &str) {
    tasks.push(Task {
        name: String::from(name),
        done: false,
    });
    println!("Додано завдання: {}", name);
}

// Редагувати завдання
pub fn edit_task(tasks: &mut Vec<Task>, index: usize, new_name: &str) {
    if let Some(task) = tasks.get_mut(index) {
        task.name = String::from(new_name);
        println!("Завдання оновлено: {}", new_name);
    } else {
        println!("Помилка: завдання з таким індексом не існує.");
    }
}

// Видалити завдання
pub fn remove_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        let removed = tasks.remove(index);
        println!("Видалено завдання: {}", removed.name);
    } else {
        println!("Помилка: завдання з таким індексом не існує.");
    }
}

// Позначити завдання як виконане
pub fn mark_done(tasks: &mut Vec<Task>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.done = true;
        println!("Завдання \"{}\" позначено як виконане.", task.name);
    } else {
        println!("Помилка: завдання з таким індексом не існує.");
    }
}
