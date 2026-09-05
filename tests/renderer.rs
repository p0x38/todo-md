use todo_md::parser::parse;
use todo_md::renderer::render;

#[test]
fn renders_document() {
    let input = r#"# TODO.md

## Completed

- [x] A
- [x] B

## Planned

- [ ] C
"#;

    let document = parse(input);
    let output = render(&document);

    assert_eq!(output, input);
}
