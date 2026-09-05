use todo_md::parser::parse;

#[test]
fn parses_sections_and_tasks() {
    let input = r#"
# TODO.md

## Completed

- [x] A
- [x] B

## Work in progress

- [x] C
- [ ] D

## Planned

- [ ] E
"#;

    let document = parse(input);

    assert_eq!(document.sections.len(), 3);

    assert_eq!(document.sections[0].title, "Completed");
    assert_eq!(document.sections[0].tasks.len(), 2);
    assert!(document.sections[0].tasks[0].completed);
    assert_eq!(document.sections[0].tasks[0].text, "A");

    assert_eq!(document.sections[1].title, "Work in progress");
    assert_eq!(document.sections[1].tasks.len(), 2);
    assert!(!document.sections[1].tasks[1].completed);

    assert_eq!(document.sections[2].title, "Planned");
    assert_eq!(document.sections[2].tasks[0].text, "E");
}
