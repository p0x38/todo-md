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

    fs::write(
        &file,
        "# TODO.md\n\n## Planned\n\n- [ ] Task\n",
    )
    .expect("failed to write TODO.md");

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
