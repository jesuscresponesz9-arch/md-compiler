use crate::parser::{AstNode, InlineNode};

/// Genera código HTML a partir de un vector de nodos AST
pub fn generate_html(ast: &[AstNode]) -> String {
    let mut html = String::new();
    
    // Envolver todo en un contenedor principal
    html.push_str("<!DOCTYPE html>\n<html lang=\"es\">\n<head>\n");
    html.push_str("  <meta charset=\"UTF-8\">\n");
    html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("  <title>Documento Markdown</title>\n");
    html.push_str("  <style>\n");
    html.push_str("    body { font-family: system-ui, sans-serif; max-width: 800px; margin: 2rem auto; padding: 0 1rem; line-height: 1.6; }\n");
    html.push_str("    pre { background: #f4f4f4; padding: 1rem; border-radius: 4px; overflow-x: auto; }\n");
    html.push_str("    code { background: #f4f4f4; padding: 0.2em 0.4em; border-radius: 3px; }\n");
    html.push_str("    pre code { background: none; padding: 0; }\n");
    html.push_str("    .alert-info { background: #e7f3ff; border-left: 4px solid #2196F3; padding: 1rem; margin: 1rem 0; }\n");
    html.push_str("    .alert-warning { background: #fff3cd; border-left: 4px solid #ffc107; padding: 1rem; margin: 1rem 0; }\n");
    html.push_str("    .alert-error { background: #f8d7da; border-left: 4px solid #dc3545; padding: 1rem; margin: 1rem 0; }\n");
    html.push_str("  </style>\n");
    html.push_str("</head>\n<body>\n\n");

    for node in ast {
        match node {
            AstNode::Heading { level, content } => {
                html.push_str(&format!("<h{level}>{content}</h{level}>\n"));
            }
            
            AstNode::Paragraph { content } => {
                html.push_str("<p>");
                for inline in content {
                    match inline {
                        InlineNode::Text(t) => html.push_str(t),
                        InlineNode::Bold(t) => html.push_str(&format!("<strong>{}</strong>", t)),
                        InlineNode::Italic(t) => html.push_str(&format!("<em>{}</em>", t)),
                        InlineNode::CodeInline(t) => html.push_str(&format!("<code>{}</code>", t)),
                    }
                }
                html.push_str("</p>\n");
            }
            
            AstNode::ListUnordered { items } => {
                html.push_str("<ul>\n");
                for item in items {
                    html.push_str(&format!("  <li>{}</li>\n", item));
                }
                html.push_str("</ul>\n");
            }
            
            AstNode::CodeBlock { language, code } => {
                let lang_class = language.as_deref().unwrap_or("");
                html.push_str(&format!("<pre><code class=\"language-{}\">", lang_class));
                // Escapar caracteres HTML especiales en el código
                html.push_str(&escape_html(code));
                html.push_str("</code></pre>\n");
            }
            
            AstNode::Alert { alert_type, content } => {
                let alert_class = match alert_type.as_str() {
                    "warning" => "alert-warning",
                    "error" => "alert-error",
                    _ => "alert-info",
                };
                html.push_str(&format!("<div class=\"{}\">\n", alert_class));
                html.push_str(&format!("<strong>{}:</strong> {}\n", 
                    alert_type.to_uppercase(), 
                    escape_html(content)
                ));
                html.push_str("</div>\n");
            }
            
            AstNode::BlankLine => {
                // Los saltos de línea significativos ya están implícitos en las etiquetas de bloque
            }
        }
    }

    html.push_str("\n</body>\n</html>");
    html
}

/// Escapa caracteres HTML especiales para prevenir XSS y errores de renderizado
fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}