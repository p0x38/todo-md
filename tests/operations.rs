use todo_md::operations::{add_task, complete_task, remove_task, uncomplete_task};
use todo_md::parser::parse;

fn document() -> todo_md::model::TodoDocument {
    parse(
        r#"# TODO.md

## Planned

- [ ] A
- [ ] B
"#,
    )
}

#[test]
fn adds_task() {
    let mut document = document();

    add_task(&mut document, "Planned", "C".to_string()).unwrap();

    assert_eq!(document.sections[0].tasks.len(), 3);
    assert_eq!(document.sections[0].tasks[2].text, "C");
}

#[test]
fn completes_task() {
    let mut document = document();

    complete_task(&mut document, "Planned", 0).unwrap();

    assert!(document.sections[0].tasks[0].completed);
}

#[test]
fn uncompletes_task() {
    let mut document = document();

    complete_task(&mut document, "Planned", 0).unwrap();
    uncomplete_task(&mut document, "Planned", 0).unwrap();

    assert!(!document.sections[0].tasks[0].completed);
}

#[test]
fn removes_task() {
    let mut document = document();

    remove_task(&mut document, "Planned", 0).unwrap();

    assert_eq!(document.sections[0].tasks.len(), 1);
    assert_eq!(document.sections[0].tasks[0].text, "B");
}

#[test]
fn preserves_unknown_markdown_when_completing_task() {
    let mut document = parse(
        r#"# TODO.md

Description of the project.

## Planned

- [ ] A

> Important note

Some extra Markdown.

## Other

- [ ] B
"#,
    );

    complete_task(&mut document, "Planned", 0).unwrap();

    let output = todo_md::renderer::render(&document);

    assert_eq!(
        output,
        r#"# TODO.md

Description of the project.

## Planned

- [x] A

> Important note

Some extra Markdown.

## Other

- [ ] B
"#
        .to_string()
            + "\n"
    );
}

#[test]
fn preserves_unknown_markdown_when_removing_task() {
    let mut document = parse(
        r#"# TODO.md

Description.

## Planned

- [ ] A
- [ ] B

> Keep this.

## Other

Random content.
"#,
    );

    remove_task(&mut document, "Planned", 0).unwrap();

    let output = todo_md::renderer::render(&document);

    assert_eq!(
        output,
        r#"# TODO.md

Description.

## Planned

- [ ] B

> Keep this.

## Other

Random content.
"#
        .to_string()
            + "\n"
    );
}
