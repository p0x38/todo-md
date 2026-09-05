use std::fs;

use clap::{Parser, Subcommand};

use crate::error::TodoError;
use crate::model::{Section, TodoDocument};
use crate::operations::{add_task, complete_task, remove_task, uncomplete_task};
use crate::parser::parse;
use crate::renderer::render;

/// TODO.md management tool.
#[derive(Debug, Parser)]
#[command(name = "todo-md")]
#[command(version)]
#[command(about = "Manage TODO.md files")]
pub struct Cli {
    /// Path to the TODO.md file.
    #[arg(short, long, default_value = "TODO.md")]
    pub file: String,

    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Available commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a task to a section.
    Add {
        /// Section name.
        #[arg(short, long)]
        section: String,

        /// Task text.
        text: String,
    },

    /// Mark a task as completed.
    Complete {
        /// Section name.
        #[arg(short, long)]
        section: String,

        /// Zero-based task index.
        index: usize,
    },

    /// Mark a task as incomplete.
    Uncomplete {
        /// Section name.
        #[arg(short, long)]
        section: String,

        /// Zero-based task index.
        index: usize,
    },

    /// Remove a task.
    Remove {
        /// Section name.
        #[arg(short, long)]
        section: String,

        /// Zero-based task index.
        index: usize,
    },
}

/// Runs the command-line interface.
pub fn run() {
    if let Err(error) = run_inner() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run_inner() -> Result<(), TodoError> {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        println!("Use --help to see available commands.");
        return Ok(());
    };

    let mut document = match fs::read_to_string(&cli.file) {
        Ok(input) => parse(&input),

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => match &command {
            Command::Add { .. } => TodoDocument {
                lines: Vec::new(),
                sections: Vec::new(),
            },

            _ => return Err(error.into()),
        },

        Err(error) => return Err(error.into()),
    };

    match command {
        Command::Add { section, text } => {
            if !document
                .sections
                .iter()
                .any(|existing| existing.title == section)
            {
                let line = document.lines.len();

                document.lines.push(format!("## {section}"));
                document.lines.push(String::new());

                document.sections.push(Section {
                    title: section.clone(),
                    line,
                    tasks: Vec::new(),
                });
            }

            add_task(&mut document, &section, text)?;
        }

        Command::Complete { section, index } => {
            complete_task(&mut document, &section, index)?;
        }

        Command::Uncomplete { section, index } => {
            uncomplete_task(&mut document, &section, index)?;
        }

        Command::Remove { section, index } => {
            remove_task(&mut document, &section, index)?;
        }
    }

    fs::write(&cli.file, render(&document))?;

    Ok(())
}
