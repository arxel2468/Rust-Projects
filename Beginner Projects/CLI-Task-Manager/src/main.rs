// CLI Task Manager in Rust

use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const FILE_PATH: &str = "tasks.txt";

fn main() {
    loop {
        println!("\nTask Manager:\n1. Add Task\n2. View Tasks\n3. Mark Task as Done\n4. Delete Task\n5. Exit");
        let choice = get_input("Choose an option: ");
        match choice.trim() {
            "1" => add_task(),
            "2" => view_tasks(),
            "3" => mark_task_done(),
            "4" => delete_task(),
            "5" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn add_task() {
    let task = get_input("Enter the task: ");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)
        .unwrap();
    writeln!(file, "[ ] {}", task.trim()).unwrap();
    println!("Task added successfully.");
}

fn view_tasks() {
    if !Path::new(FILE_PATH).exists() {
        println!("No tasks found.");
        return;
    }

    let file = fs::File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);

    println!("\nTasks:");
    for (index, line) in reader.lines().enumerate() {
        println!("{}. {}", index + 1, line.unwrap());
    }
}

fn mark_task_done() {
    if !Path::new(FILE_PATH).exists() {
        println!("No tasks found.");
        return;
    }

    view_tasks();
    let task_number = get_input("Enter the task number to mark as done: ");
    let task_number: usize = match task_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    let file = fs::File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);
    let mut tasks: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    if task_number == 0 || task_number > tasks.len() {
        println!("Invalid task number.");
        return;
    }

    tasks[task_number - 1] = tasks[task_number - 1].replacen("[ ]", "[X]", 1);

    fs::write(FILE_PATH, tasks.join("\n") + "\n").unwrap();
    println!("Task marked as done.");
}

fn delete_task() {
    if !Path::new(FILE_PATH).exists() {
        println!("No tasks found.");
        return;
    }

    view_tasks();
    let task_number = get_input("Enter the task number to delete: ");
    let task_number: usize = match task_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    let file = fs::File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);
    let mut tasks: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    if task_number == 0 || task_number > tasks.len() {
        println!("Invalid task number.");
        return;
    }

    tasks.remove(task_number - 1);

    fs::write(FILE_PATH, tasks.join("\n") + "\n").unwrap();
    println!("Task deleted successfully.");
}
