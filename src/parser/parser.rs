use crate::lexer::{Token, TokenType};
use super::ast_types::{AstNode, InlineNode};

/// El Parser seguro: convierte tokens en AST con verificación de límites
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Obtiene el token actual de forma segura
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Avanza el cursor y devuelve el token anterior
    fn consume(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        token
    }

    /// Parsea todos los tokens y devuelve el AST completo
    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut ast = Vec::new();

        while let Some(token) = self.current() {
            let token_type = token.token_type.clone();

            match token_type {
                TokenType::Heading(level) => {
                    if let Some(t) = self.consume() {
                        ast.push(AstNode::Heading { 
                            level, 
                            content: t.value.clone() 
                        });
                    }
                }
                
                TokenType::ListUnordered => {
                    let mut items = Vec::new();
                    while let Some(t) = self.current() {
                        if matches!(t.token_type, TokenType::ListUnordered) {
                            if let Some(item_token) = self.consume() {
                                items.push(item_token.value.clone());
                            }
                        } else {
                            break;
                        }
                    }
                    if !items.is_empty() {
                        ast.push(AstNode::ListUnordered { items });
                    }
                }
                
                // CORRECCIÓN: Usar struct variant con campos nombrados
                TokenType::CodeBlock { language, content } => {
                    if let Some(_t) = self.consume() {
                        ast.push(AstNode::CodeBlock {
                            language: if language.is_empty() { None } else { Some(language) },
                            code: content,
                        });
                    }
                }
                
                TokenType::AlertStart(alert_type) => {
                    self.consume(); // Consumir AlertStart
                    
                    let mut content_parts = Vec::new();
                    while let Some(t) = self.current() {
                        if matches!(t.token_type, TokenType::AlertEnd) {
                            self.consume();
                            break;
                        }
                        
                        if matches!(t.token_type, TokenType::Paragraph) {
                            if let Some(p) = self.consume() {
                                content_parts.push(p.value.clone());
                            }
                        } else {
                            self.consume();
                        }
                    }
                    
                    ast.push(AstNode::Alert {
                        alert_type: alert_type.clone(),
                        content: content_parts.join("\n"),
                    });
                }
                
                // CORRECCIÓN: CodeInline ahora es tuple variant
                TokenType::Paragraph | TokenType::Bold | TokenType::Italic | TokenType::CodeInline(_) => {
                    let mut inlines = Vec::new();
                    
                    while let Some(t) = self.current() {
                        let tt = t.token_type.clone();
                        // CORRECCIÓN: Usar patrón con wildcard para CodeInline
                        if matches!(tt, TokenType::Paragraph | TokenType::Bold | TokenType::Italic | TokenType::CodeInline(_)) {
                            if let Some(inline_token) = self.consume() {
                                match &inline_token.token_type {
                                    TokenType::Paragraph => inlines.push(InlineNode::Text(inline_token.value.clone())),
                                    TokenType::Bold => inlines.push(InlineNode::Bold(inline_token.value.clone())),
                                    TokenType::Italic => inlines.push(InlineNode::Italic(inline_token.value.clone())),
                                    // CORRECCIÓN: Extraer contenido del tuple variant
                                    TokenType::CodeInline(code) => inlines.push(InlineNode::CodeInline(code.clone())),
                                    _ => {}
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    
                    if !inlines.is_empty() {
                        ast.push(AstNode::Paragraph { content: inlines });
                    }
                }
                
                TokenType::Newline => {
                    self.consume();
                }
                
                TokenType::Eof | TokenType::AlertEnd => {
                    self.consume();
                    if matches!(token_type, TokenType::Eof) {
                        break;
                    }
                }
                
                _ => {
                    self.consume();
                }
            }
        }

        ast
    }
}