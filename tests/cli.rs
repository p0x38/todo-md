use std::fs;
use std::process::Command;

fn todo_md_binary() -> &'static str {
    env!("CARGO_BIN_EXE_todo-md")
}

#[test]
fn shows_help() {
    let output = Command::new(todo_md_binary())
        .arg("--help")
        .output()
        .expect("failed to run todo-md");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Manage TODO.md files"));
    assert!(stdout.contains("add"));
    assert!(stdout.contains("complete"));
    assert!(stdout.contains("remove"));
}

#[test]
fn add_creates_missing_file() {
    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let file = directory.path().join("TODO.md");

    let output = Command::new(todo_md_binary())
        .arg("--file")
        .arg(&file)
        .arg("add")
        .arg("--section")
        .arg("Planned")
        .arg("Write some Rust")
        .output()
        .expect("failed to run todo-md");

    assert!(output.status.success());

    let contents = fs::read_to_string(&file).expect("failed to read TODO.md");

    assert_eq!(contents, "## Planned\n\n- [ ] Write some Rust\n");
}

#[test]
fn complete_reports_missing_section() {
    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let file = directory.path().join("TODO.md");

    fs::write(&file, "# TODO.md\n\n## Planned\n\n- [ ] Task\n").expect("failed to write TODO.md");

    let output = Command::new(todo_md_binary())
        .arg("--file")
        .arg(&file)
        .arg("complete")
        .arg("--section")
        .arg("Missing")
        .arg("0")
        .output()
        .expect("failed to run todo-md");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("section not found"));
}

#[test]
fn lists_tasks() {
    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let file = directory.path().join("TODO.md");

    fs::write(
        &file,
        "# TODO.md\n\n\
         ## Planned\n\n\
         - [ ] First task\n\
         - [x] Second task\n\n\
         ## Other\n\n\
         - [ ] Other task\n",
    )
    .expect("failed to write TODO.md");

    let output = Command::new(todo_md_binary())
        .arg("--file")
        .arg(&file)
        .arg("list")
        .output()
        .expect("failed to run todo-md");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Planned"));
    assert!(stdout.contains("[ ] First task"));
    assert!(stdout.contains("[x] Second task"));
    assert!(stdout.contains("Other"));
    assert!(stdout.contains("[ ] Other task"));
}

#[test]
fn lists_only_selected_section() {
    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let file = directory.path().join("TODO.md");

    fs::write(
        &file,
        "# TODO.md\n\n\
         ## Planned\n\n\
         - [ ] Planned task\n\n\
         ## Other\n\n\
         - [ ] Other task\n",
    )
    .expect("failed to write TODO.md");

    let output = Command::new(todo_md_binary())
        .arg("--file")
        .arg(&file)
        .arg("list")
        .arg("--section")
        .arg("Planned")
        .output()
        .expect("failed to run todo-md");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Planned"));
    assert!(stdout.contains("[ ] Planned task"));
    assert!(!stdout.contains("Other"));
    assert!(!stdout.contains("Other task"));
}

#[test]
fn validate_rejects_invalid_header() {
    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let file = directory.path().join("TODO.md");

    fs::write(
        &file,
        "# Wrong Header\n\n\
         ## Planned\n\n\
         - [ ] Task\n",
    )
    .expect("failed to write TODO.md");

    let output = Command::new(todo_md_binary())
        .arg("--file")
        .arg(&file)
        .arg("validate")
        .output()
        .expect("failed to run todo-md");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("line 1"));
    assert!(stderr.contains("TODO.md"));
}

#[test]
fn validate_rejects_malformed_task() {
    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let file = directory.path().join("TODO.md");

    fs::write(
        &file,
        "# TODO.md\n\n\
         ## Planned\n\n\
         - [ ] Valid task\n\
         - [?] Invalid task\n",
    )
    .expect("failed to write TODO.md");

    let output = Command::new(todo_md_binary())
        .arg("--file")
        .arg(&file)
        .arg("validate")
        .output()
        .expect("failed to run todo-md");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("line 6"));
    assert!(stderr.contains("invalid task status"));
}

#[test]
fn validate_allows_unknown_markdown() {
    let directory = tempfile::tempdir().expect("failed to create temporary directory");
    let file = directory.path().join("TODO.md");

    fs::write(
        &file,
        "# TODO.md\n\n\
         Project description.\n\n\
         ## Planned\n\n\
         > Important note\n\n\
         - [ ] Valid task\n",
    )
    .expect("failed to write TODO.md");

    let output = Command::new(todo_md_binary())
        .arg("--file")
        .arg(&file)
        .arg("validate")
        .output()
        .expect("failed to run todo-md");

    assert!(output.status.success());
}
