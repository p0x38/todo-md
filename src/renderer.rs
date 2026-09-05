use crate::model::TodoDocument;

/// Renders a TODO document back into Markdown.
pub fn render(document: &TodoDocument) -> String {
    let mut output = document.lines.join("\n");

    if !output.is_empty() {
        output.push('\n');
    }

    output
}
