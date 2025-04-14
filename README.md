# 📝 Rust Todo CLI

A simple and minimal Command Line Todo App built with Rust using `clap` and `serde`.

## 🚀 Features

- Add todo items
- Mark items as done
- Delete todo items
- Edit existing todo items
- List all todos
- Show a todo
- Clear the todos
- Data saved in `todos.json`

## 📦 Dependencies

- [clap](https://docs.rs/clap/) for command-line argument parsing
- [serde](https://serde.rs/) and `serde_json` for reading/writing JSON

## 🔧 Installation

Clone the repo:

```bash
git clone https://github.com/Yash981/rust-cli-todo-app.git
cd rust-cli-todo-app
```
Build:
```bash
cargo build --release
```
Test:
```bash
cp ./target/release/todo ~/.cargo/bin/
```
Make sure ```~/.cargo/bin/``` is in your ```$PATH```

🧪 Usage
```bash
todo add "Learn Rust"
todo list
todo show 0
todo done 0
todo delete 0
todo edit 1 "Learn advanced Rust"
todo clear
```

📂 Todos Storage

All todos are saved to a local file: ```todos.json```
