use clap::{Parser,Subcommand};
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
}
#[derive(Debug)]
struct Todo {
    id: usize,
    text: String,
    done: bool
}
fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    
    let args = Cli::parse();


    match args.command {
        Commands::Add { text } =>{
            let id = todos.len();
            todos.push(Todo { id: id, text: text, done: false });
            print!("Todo Added")
        },
        Commands::List => {
            println!("Todos:,{:?}",todos);
            for todo in &todos {
                println!(
                    "{}. [{}] {}",todo.id,if todo.done { "x" } else { " " },todo.text);
            }
        },
        Commands::Done { id } => {
            if let Some(todo) = todos.get_mut(id){
                todo.done = true;
                println!("Marked Todo {} id as done.", id)
            } else {
                println!("Todo with {} id is not found.",id)
            }
        },
        Commands::Delete { id } => {
            if id < todos.len() {
                todos.remove(id);
                println!("Deleted a todo with {} id",id)
            }else{
                println!("Todo with {} id is not found.",id)
            }
        }
        
    }
}
