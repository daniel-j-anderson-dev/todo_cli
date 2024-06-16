mod cli;
mod todos;

use crate::{cli::*, todos::*};
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut todos = Todos::from_csv_file("todos.csv")?;

    match Configuration::parse().command() {
        Command::Add { tasks } => todos.add_todos(tasks),
        Command::Remove { id } => todos.remove_todo(id),
        Command::Done { id } => todos.complete_todo(id),
        Command::List => println!("{todos}"),
        Command::Sort => todos.sort_by_is_complete(),
        Command::Reset => todos = Todos::default(),
        Command::Restore { todo_path } => todos = Todos::from_csv_file(todo_path)?,
        Command::Raw(RawCommands::Done) => todos
            .all_completed_todos()
            .for_each(|todo| println!("{}", todo)),
        Command::Raw(RawCommands::Todo) => todos
            .all_incomplete_todos()
            .for_each(|todo| println!("{}", todo)),
    }

    todos.save("todos.csv")?;

    return Ok(());
}
