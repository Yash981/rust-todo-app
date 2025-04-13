use clap::{Parser,Subcommand};
use serde::{Deserialize, Serialize};
use std::io::{Read,Write};
use std::fs::File;
use serde_json;

#[derive(Parser, Debug)]
#[command(name = "todo", version = "1.0", about = "A simple todo CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand,Debug)]
enum Commands {
    Add { text: String },
    List,
    Done { id: usize },
    Delete { id: usize },
}
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    text: String,
    done: bool
}
fn save_todos(todos: &Vec<Todo>, path: &str) -> Result<(),Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(todos)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
fn load_todos(path: &str) -> Result<Vec<Todo>,Box<dyn std::error::Error>> {
    let mut file = File::open(path)?; 
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let todos: Vec<Todo> = serde_json::from_str(&contents)?; 
    Ok(todos)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let args = Cli::parse();


    match args.command {
        Commands::Add { text } =>{
            let mut todos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            let id = todos.len();
            todos.push(Todo { id: id, text: text, done: false });
            save_todos(&todos, "todos.json")
        },
        Commands::List => {
            let todos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            for todo in &todos {
                println!(
                    "{}. [{}] {}",todo.id,if todo.done { "x" } else { " " },todo.text);
            }
            Ok(())
        },
        Commands::Done { id } => {
            let mut todos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            if let Some(todo) = todos.get_mut(id){
                todo.done = true;
                println!("Marked Todo {} id as done.", id);
                save_todos(&todos, "todos.json")
            } else {
                println!("Todo with {} id is not found.",id);
                Ok(())
            }
        },
        Commands::Delete { id } => {
            let mut todos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            if id < todos.len() {
                todos.remove(id);
                println!("Deleted a todo with {} id",id);
                save_todos(&todos, "todos.json")
            }else{
                println!("Todo with {} id is not found.",id);
                Ok(())
            }
        }
        
    }
}
