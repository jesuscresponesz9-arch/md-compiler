/// Representa cada tipo de token que puede encontrar el lexer
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Estructura básica
    Heading(u8),        // # ## ### (nivel 1-6)
    Paragraph,          // Texto normal
    Bold,               // **texto**
    Italic,             // *texto*
    CodeInline(String), // `código` (ahora guarda el contenido)
    CodeBlock {         // ```bloque``` (estructura clara)
        language: String,
        content: String,
    },
    ListUnordered,      // - item
    ListOrdered,        // 1. item
    
    // Extensiones custom
    AlertStart(String), // :::info, :::warning
    AlertEnd,           // ::: (cierre)
    
    // Control
    Newline,
    Eof,
    Unknown(String),
}

/// Un token individual con su valor y posición
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,   // Valor legible para debug/visualización
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            value,
            line,
            column,
        }
    }
}