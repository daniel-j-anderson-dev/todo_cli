use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Configuration {
    #[command(subcommand)]
    command: Command,
}
impl Configuration {
    pub fn command(&self) -> Command {
        return self.command.clone();
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Add {
        tasks: Vec<String>,
    },
    Remove {
        id: usize,
    },
    Done {
        id: usize,
    },
    List,
    Sort,
    Reset,
    Restore {
        todo_path: PathBuf,
    },
    #[command(subcommand)]
    Raw(RawCommands),
}

#[derive(Debug, Clone, Subcommand)]
pub enum RawCommands {
    Done,
    Todo,
}
