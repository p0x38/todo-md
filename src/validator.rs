use crate::error::TodoError;
use crate::model::TodoDocument;

pub fn validate(document: &TodoDocument) -> Result<(), TodoError> {
    validate_header(document)?;
    validate_tasks(document)?;

    Ok(())
}

fn validate_header(document: &TodoDocument) -> Result<(), TodoError> {
    let Some(first_line) = document.lines.first() else {
        return Err(TodoError::Validation {
            line: 1,
            message: "missing TODO.md header".to_string(),
        });
    };

    let header = first_line.trim();

    if header != "# TODO" && header != "# TODO.md" {
        return Err(TodoError::Validation {
            line: 1,
            message: "first line must be '# TODO' or '# TODO.md'".to_string(),
        });
    }

    Ok(())
}

fn validate_tasks(document: &TodoDocument) -> Result<(), TodoError> {
    for (line_number, line) in document.lines.iter().enumerate() {
        let trimmed = line.trim();

        if !trimmed.starts_with("- [") {
            continue;
        }

        let Some(task) = trimmed.strip_prefix("- [") else {
            continue;
        };

        let Some((status, _)) = task.split_once("] ") else {
            return Err(TodoError::Validation {
                line: line_number + 1,
                message: "malformed task".to_string(),
            });
        };

        if status != " " && status != "x" && status != "X" {
            return Err(TodoError::Validation {
                line: line_number + 1,
                message: format!("invalid task status '{status}'"),
            });
        }
    }

    Ok(())
}
