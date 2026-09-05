use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("section not found: {0}")]
    SectionNotFound(String),

    #[error("task {index} not found in section '{section}'")]
    TaskNotFound { section: String, index: usize },

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}
