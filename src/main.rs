use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
#[derive(Parser, Debug)]
#[command(name = "todo", version = "1.0", about = "A simple todo CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { text: String },
    List,
    Done { id: usize },
    Delete { id: usize },
    Clear,
    Show {id: usize},
    Edit {id:usize,text:String},
    Undone { id: usize },
}
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    text: String,
    done: bool,
}
fn save_todos(todos: &Vec<Todo>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(todos)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
fn load_todos(path: &str) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let todos: Vec<Todo> = serde_json::from_str(&contents)?;
    Ok(todos)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Add { text } => {
            let mut todos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            let id = todos.len();
            todos.push(Todo {
                id: id,
                text: text,
                done: false,
            });
            save_todos(&todos, "todos.json")
        }
        Commands::List => {
            let todos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            for todo in &todos {
                println!(
                    "{}. [{}] {}",
                    todo.id,
                    if todo.done { "x" } else { " " },
                    todo.text
                );
            }
            Ok(())
        }
        Commands::Done { id } => {
            let mut todos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            if let Some(todo) = todos.get_mut(id) {
                todo.done = true;
                println!("Marked Todo {} id as done.", id);
                save_todos(&todos, "todos.json")
            } else {
                println!("Todo with {} id is not found.", id);
                Ok(())
            }
        }
        Commands::Delete { id } => {
            let todos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            let mut new_vec: Vec<Todo> = Vec::new();
            let mut perform = false;
            for x in todos {
                if x.id == id {
                    perform = true
                } else {
                    new_vec.push(x);
                }
            }
            if perform {
                println!("Deleted a todo with {} id", id);
                return save_todos(&new_vec, "todos.json");
            }
            println!("Todo with {} id is not found.", id);
            Ok(())
        }
        Commands::Clear => {
            let path = Path::new("todos.json");
            if !path.exists() {
                println!("Already Cleared the todos,No Todos exist to Clear");
                Ok(())
            } else {
                let res = remove_file("todos.json");
                match res {
                    Ok(_val) => {
                        println!("Cleared the todos");
                        Ok(())
                    }
                    Err(_) => {
                        println!("Having issue clearing the todos");
                        Ok(())
                    }
                }
            }
        }
        Commands::Show {id  } => {
            let todos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            for todo in todos{
                if todo.id == id{
                    println!("Todo id: {}",id);
                    println!("Todo Text: {}",todo.text);
                    println!("Todo Marked: {}",todo.done);
                    return Ok(());
                }
            }
            println!("Todo with {} id doesn't exist to show",id);
            return Ok(());
        }
        Commands::Edit { id, text } => {
            let mut loadedtodos = load_todos("todos.json").unwrap_or_else(|_| Vec::new());
            let mut edited = false;
            for todo in loadedtodos.iter_mut(){
                if todo.id == id{
                    todo.text = text;
                    edited = true;
                    break
                }
            }
            if edited {
                println!("successfully edited the text for the {}",id);
                save_todos(&loadedtodos, "todos.json")?;
                return Ok(());
            }
            println!("ID:{} doesn't exist in todolist",id);
            Ok(())

        }
        Commands::Undone { id } =>{
            let mut todos = load_todos("todos.json").unwrap_or_else(|_| vec![]);
            for todo in todos.iter_mut(){
                if todo.id == id && !todo.done {
                    println!("Already it is undone");
                    return Ok(());
                }else if todo.id == id && todo.done {
                    todo.done = false;
                    println!("Marked undone for todo id:{}",id);
                    save_todos(&todos, "todos.json")?;
                    return Ok(());
                }
            }
            println!("No todo exist with id:{} in the todos",id);
            Ok(())
        }
    }
}
