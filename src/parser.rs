use crate::model::{Section, Task, TodoDocument};

pub fn parse(input: &str) -> TodoDocument {
    let lines: Vec<String> = input.lines().map(str::to_owned).collect();

    let mut document = TodoDocument {
        lines,
        sections: Vec::new(),
    };

    let mut current_section: Option<usize> = None;

    for line_number in 0..document.lines.len() {
        let line = &document.lines[line_number];
        let trimmed = line.trim();

        if let Some(title) = trimmed.strip_prefix("## ") {
            document.sections.push(Section {
                title: title.to_string(),
                line: line_number,
                tasks: Vec::new(),
            });

            current_section = Some(document.sections.len() - 1);
            continue;
        }

        let Some(task) = parse_task(trimmed, line_number) else {
            continue;
        };

        if let Some(section_index) = current_section {
            document.sections[section_index].tasks.push(task);
        }
    }

    document
}

fn parse_task(line: &str, line_number: usize) -> Option<Task> {
    let task = line.strip_prefix("- [")?;
    let (status, text) = task.split_once("] ")?;

    let completed = match status {
        " " => false,
        "x" | "X" => true,
        _ => return None,
    };

    Some(Task {
        completed,
        text: text.to_string(),
        line: line_number,
    })
}
