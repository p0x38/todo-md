//! Data structures representing a TODO.md document.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoDocument {
    pub lines: Vec<String>,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub title: String,
    pub line: usize,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub completed: bool,
    pub text: String,
    pub line: usize,
}

impl TodoDocument {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            lines: Vec::new(),
        }
    }
}

impl Default for TodoDocument {
    fn default() -> Self {
        Self::new()
    }
}
