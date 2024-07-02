use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    task: String,
    done: bool,
}

impl Todo {
    fn new(task: String) -> Todo {
        Todo { task, done: false }
    }
}

fn main() {
    let matches = Command::new("To-Do List")
        .version("1.1")
        .author("Harshit < https://github.com/harshitkumar9030 >")
        .about("Manages a to-do list")
        .arg(
            Arg::new("add")
                .short('a')
                .long("add")
                .value_name("TASK")
                .help("Adds a new task")
                .takes_value(true),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("Lists all tasks"),
        )
        .arg(
            Arg::new("done")
                .short('d')
                .long("done")
                .value_name("INDEX")
                .help("Marks a task as done")
                .takes_value(true),
        )
        .arg(
            Arg::new("undone")
                .short('u')
                .long("undone")
                .value_name("INDEX")
                .help("Marks a task as undone")
                .takes_value(true),
        )
        .arg(
            Arg::new("remove")
                .short('r')
                .long("remove")
                .value_name("INDEX")
                .help("Removes a task")
                .takes_value(true),
        )
        .arg(
            Arg::new("edit")
                .short('e')
                .long("edit")
                .value_name("INDEX,TASK")
                .help("Edits a task")
                .takes_value(true),
        )
        .get_matches();

    let file_path = "todo.json";

    let mut todos: Vec<Todo> = if let Ok(data) = fs::read_to_string(file_path) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    if let Some(task) = matches.get_one::<String>("add") {
        todos.push(Todo::new(task.to_string()));
        save_todos(file_path, &todos).expect("Failed to save tasks.");
        println!("Added: {}", task);
    } else if matches.contains_id("list") {
        for (index, todo) in todos.iter().enumerate() {
            println!("{}: {} [{}]", index + 1, todo.task, if todo.done { "x" } else { " " });
        }
    } else if let Some(index) = matches.get_one::<String>("done") {
        if let Ok(index) = index.parse::<usize>() {
            if index > 0 && index <= todos.len() {
                todos[index - 1].done = true;
                save_todos(file_path, &todos).expect("Failed to save tasks.");
                println!("Marked task {} as done", index);
            } else {
                println!("Invalid task index");
            }
        }
    } else if let Some(index) = matches.get_one::<String>("undone") {
        if let Ok(index) = index.parse::<usize>() {
            if index > 0 && index <= todos.len() {
                todos[index - 1].done = false;
                save_todos(file_path, &todos).expect("Failed to save tasks.");
                println!("Marked task {} as undone", index);
            } else {
                println!("Invalid task index");
            }
        }
    } else if let Some(index) = matches.get_one::<String>("remove") {
        if let Ok(index) = index.parse::<usize>() {
            if index > 0 && index <= todos.len() {
                todos.remove(index - 1);
                save_todos(file_path, &todos).expect("Failed to save tasks.");
                println!("Removed task {}", index);
            } else {
                println!("Invalid task index");
            }
        }
    } else if let Some(edit) = matches.get_one::<String>("edit") {
        let parts: Vec<&str> = edit.splitn(2, ',').collect();
        if parts.len() == 2 {
            if let Ok(index) = parts[0].parse::<usize>() {
                if index > 0 && index <= todos.len() {
                    todos[index - 1].task = parts[1].to_string();
                    save_todos(file_path, &todos).expect("Failed to save tasks.");
                    println!("Edited task {} to '{}'", index, parts[1]);
                } else {
                    println!("Invalid task index");
                }
            }
        } else {
            println!("Invalid edit format. Use -e INDEX,TASK");
        }
    }
}

fn save_todos(file_path: &str, todos: &Vec<Todo>) -> io::Result<()> {
    let json = serde_json::to_string(&todos)?;
    let mut file = OpenOptions::new().write(true).create(true).open(file_path)?;
    file.set_len(0)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
