use crate::error::TodoError;
use crate::model::{Task, TodoDocument};

pub fn add_task(
    document: &mut TodoDocument,
    section_title: &str,
    text: String,
) -> Result<(), TodoError> {
    let section_index = document
        .sections
        .iter()
        .position(|section| section.title == section_title)
        .ok_or_else(|| TodoError::SectionNotFound(section_title.to_string()))?;

    let section_start = document.sections[section_index].line;
    let section_end = document
        .sections
        .get(section_index + 1)
        .map(|section| section.line)
        .unwrap_or(document.lines.len());

    let insert_at = if let Some(task) = document.sections[section_index].tasks.last() {
        task.line + 1
    } else {
        let content_end = document.lines[section_start + 1..section_end]
            .iter()
            .rposition(|line| !line.trim().is_empty());

        match content_end {
            Some(index) => section_start + index + 2,
            None => section_start + 2,
        }
    };

    document.lines.insert(insert_at, format!("- [ ] {text}"));

    rebuild(document);

    Ok(())
}

pub fn complete_task(
    document: &mut TodoDocument,
    section_title: &str,
    task_index: usize,
) -> Result<(), TodoError> {
    let (line, text) = {
        let task = find_task(document, section_title, task_index)?;
        (task.line, task.text.clone())
    };

    document.lines[line] = format!("- [x] {text}");

    rebuild(document);

    Ok(())
}

pub fn uncomplete_task(
    document: &mut TodoDocument,
    section_title: &str,
    task_index: usize,
) -> Result<(), TodoError> {
    let (line, text) = {
        let task = find_task(document, section_title, task_index)?;
        (task.line, task.text.clone())
    };

    document.lines[line] = format!("- [ ] {text}");

    rebuild(document);

    Ok(())
}

pub fn remove_task(
    document: &mut TodoDocument,
    section_title: &str,
    task_index: usize,
) -> Result<(), TodoError> {
    let line = {
        let task = find_task(document, section_title, task_index)?;
        task.line
    };

    document.lines.remove(line);

    rebuild(document);

    Ok(())
}

fn find_task<'a>(
    document: &'a TodoDocument,
    section_title: &str,
    task_index: usize,
) -> Result<&'a Task, TodoError> {
    let section = document
        .sections
        .iter()
        .find(|section| section.title == section_title)
        .ok_or_else(|| TodoError::SectionNotFound(section_title.to_string()))?;

    section
        .tasks
        .get(task_index)
        .ok_or_else(|| TodoError::TaskNotFound {
            section: section_title.to_string(),
            index: task_index,
        })
}

fn rebuild(document: &mut TodoDocument) {
    let parsed = crate::parser::parse(&document.lines.join("\n"));
    document.sections = parsed.sections;
}
