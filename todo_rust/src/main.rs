/*
 * What is the difference between
 * (todo: &ToDo) and (todos: &mut Vec<ToDo>) and (mut file: &File)?
 *  */
use chrono::prelude::*;
use std::{
    fs::{write, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

// struct Data(i32, i32);
// Data.0, Data.1

#[derive(Debug)]
struct ToDo {
    title: String,
    completed: bool,
    create_at: String,
}

trait ToDoTrait {
    fn mark_todo_as_completed(&mut self);
}

impl ToDoTrait for ToDo {
    fn mark_todo_as_completed(&mut self) {
        self.completed = true;
    }
}

impl ToDo {
    pub fn new(title: String, completed: bool, create_at: String) -> Self {
        Self {
            title,
            completed,
            create_at,
        }
    }
    pub fn clean_todos_data_txt(path: &Path) {
        write(path, "").expect("Failed to write to file");
    }
    pub fn convert_todo_string_to_todo(todo_string: &str, index: u8) -> ToDo {
        let todo_string_split: Vec<&str> = todo_string.split("|").collect();
        let todo_title = todo_string_split[2]
            .replace("To do title:", "")
            .trim()
            .to_string();
        let todo_completed = todo_string_split[0]
            .replace(&format!("{}. Status: ", index + 1), "")
            .trim()
            .to_string()
            == "DONE";
        let todo_create_at = todo_string_split[1]
            .replace("Created At: ", "")
            .trim()
            .to_string();
        return ToDo::new(todo_title, todo_completed, todo_create_at);
    }
    pub fn convert_todo_to_string(index: u8, todo: &ToDo) -> String {
        let mut status = "DONE";
        if todo.completed == false {
            status = "NOT DONE";
        }
        return format!(
            "{}. Status: {} | Created At: {} | To do title: {}",
            index + 1,
            status,
            todo.create_at,
            todo.title
        );
    }
    pub fn update_todo_to_file(todos: &Vec<ToDo>, mut file: &File, path: &Path) {
        ToDo::clean_todos_data_txt(path);
        for (index, todo) in todos.iter().enumerate() {
            write!(
                file,
                "{}\n",
                ToDo::convert_todo_to_string(index as u8, todo)
            )
            .expect("Failed to write to file");
        }
    }
}

fn load_data_to_vec(file: &mut File, todos: &mut Vec<ToDo>) {
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let tododetail = line.unwrap();
        todos.push(ToDo::convert_todo_string_to_todo(&tododetail, index as u8));
    }
}

fn main() {
    let mut run = true;
    let path = Path::new(".\\src\\todos.txt");
    if !path.exists() {
        write(path, "").expect("Failed to write to file");
    }
    let mut todos: Vec<ToDo> = Vec::new();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");
    load_data_to_vec(&mut file, &mut todos);
    while run != false {
        println!("All to do: \n");
        for (index, todo) in todos.iter().enumerate() {
            println!("{}", ToDo::convert_todo_to_string(index as u8, todo));
        }
        println!("\nAll action\n");
        println!("1. Add To Do");
        println!("2. Delete To Do");
        println!("3. Mark To Do as completed");
        println!("4. Exit");
        println!("Enter your action: ");
        let mut action: String = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        match action.trim() {
            "4" => {
                run = false;
            }
            "1" => {
                println!("Enter To Do title: ");
                let mut title: String = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line");
                title = title.trim().to_string();
                let new_to_do = ToDo::new(title, false, Utc::now().to_string());
                todos.push(new_to_do);
                ToDo::update_todo_to_file(&todos, &file, &path);
                println!("")
            }
            "2" => {
                println!("Enter To Do index: ");
                let mut index: String = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");
                let index: usize = index.trim().parse().expect("Please enter a number");
                if index > todos.len() {
                    println!("Index out of range");
                } else {
                    todos.remove(index - 1);
                    ToDo::update_todo_to_file(&todos, &file, &path);
                }
                println!("")
            }
            "3" => {
                println!("Enter To Do index: ");
                let mut index: String = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");
                let index: usize = index.trim().parse().expect("Please enter a number");
                if index > todos.len() {
                    println!("Index out of range");
                } else {
                    todos[index - 1].mark_todo_as_completed();
                    ToDo::update_todo_to_file(&todos, &file, &path);
                }
                println!("")
            }
            _ => {
                println!("Invalid action");
            }
        }
    }
}
