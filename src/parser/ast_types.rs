/// Nodos del Árbol de Sintaxis Abstracta (AST)
/// Representan la estructura lógica del documento Markdown

#[derive(Debug, Clone)]
pub enum AstNode {
    /// Un título (# ## ###)
    Heading {
        level: u8,
        content: String,
    },
    
    /// Un párrafo de texto plano o con énfasis inline
    Paragraph {
        content: Vec<InlineNode>,
    },
    
    /// Una lista no ordenada (- item)
    ListUnordered {
        items: Vec<String>,
    },
    
    /// Un bloque de código (```)
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    
    /// Una alerta personalizada (:::info ... :::)
    Alert {
        alert_type: String,
        content: String,
    },
    
    /// Espaciado / salto de línea significativo
    BlankLine,
}

/// Elementos inline dentro de un párrafo (bold, italic, texto)
#[derive(Debug, Clone)]
pub enum InlineNode {
    Text(String),
    Bold(String),
    Italic(String),
    CodeInline(String),
}