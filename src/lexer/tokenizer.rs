use super::token_types::{Token, TokenType};

/// El Lexer mejorado: convierte texto Markdown en tokens
pub struct Tokenizer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        // CORRECCIÓN CRÍTICA: Normalizar \r\n (Windows) a \n (Unix)
        let normalized = input.replace("\r\n", "\n").replace('\r', "\n");
        
        Self {
            input: normalized.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokeniza todo el input
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.input.len() {
            let current = self.current_char();

            match current {
                '#' => tokens.push(self.tokenize_heading()),
                '*' => tokens.push(self.tokenize_emphasis()),
                '`' => tokens.push(self.tokenize_code()),
                '-' => tokens.push(self.tokenize_list_unordered()),
                ':' => tokens.push(self.try_tokenize_alert()),
                '\n' => {
                    tokens.push(Token::new(TokenType::Newline, "\n".to_string(), self.line, self.column));
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                _ => {
                    let start_line = self.line;
                    let start_col = self.column;
                    let mut text = String::new();

                    while self.pos < self.input.len()
                        && !matches!(self.current_char(), '#' | '*' | '`' | '-' | ':' | '\n')
                    {
                        text.push(self.current_char());
                        self.advance();
                        self.column += 1;
                    }

                    if !text.trim().is_empty() {
                        tokens.push(Token::new(TokenType::Paragraph, text, start_line, start_col));
                    } else if !text.is_empty() {
                        // Consumir espacios sin crear token
                    }
                }
            }
        }

        tokens.push(Token::new(TokenType::Eof, "".to_string(), self.line, self.column));
        tokens
    }

    fn tokenize_heading(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;
        let mut level = 0u8;

        while self.pos < self.input.len() && self.current_char() == '#' && level < 6 {
            level += 1;
            self.advance();
            self.column += 1;
        }

        if self.pos < self.input.len() && self.current_char() == ' ' {
            self.advance();
            self.column += 1;
        }

        let mut title = String::new();
        while self.pos < self.input.len() && self.current_char() != '\n' {
            title.push(self.current_char());
            self.advance();
            self.column += 1;
        }

        Token::new(TokenType::Heading(level), title, start_line, start_col)
    }

    fn tokenize_emphasis(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;

        self.advance();
        self.column += 1;

        if self.pos < self.input.len() && self.current_char() == '*' {
            // Bold: **texto**
            self.advance();
            self.column += 1;

            let mut content = String::new();
            while self.pos < self.input.len()
                && !(self.current_char() == '*' && self.peek_next() == Some('*'))
            {
                content.push(self.current_char());
                self.advance();
                self.column += 1;
            }

            if self.pos < self.input.len() {
                self.advance();
                self.column += 1;
                self.advance();
                self.column += 1;
            }

            Token::new(TokenType::Bold, content, start_line, start_col)
        } else {
            // Italic: *texto*
            let mut content = String::new();
            while self.pos < self.input.len() && self.current_char() != '*' {
                content.push(self.current_char());
                self.advance();
                self.column += 1;
            }

            if self.pos < self.input.len() {
                self.advance();
                self.column += 1;
            }

            Token::new(TokenType::Italic, content, start_line, start_col)
        }
    }

    fn tokenize_code(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;

        self.advance(); // consumir primer `
        self.column += 1;

        if self.pos < self.input.len() && self.current_char() == '`' {
            self.advance();
            self.column += 1;
            
            if self.pos < self.input.len() && self.current_char() == '`' {
                self.advance();
                self.column += 1;
                
                // Leer lenguaje opcional (trim automático)
                let mut lang = String::new();
                while self.pos < self.input.len() 
                    && self.current_char() != '\n' 
                    && self.current_char() != '`' 
                {
                    lang.push(self.current_char());
                    self.advance();
                    self.column += 1;
                }
                let lang = lang.trim().to_string();
                
                // Consumir newline después del lenguaje
                if self.pos < self.input.len() && self.current_char() == '\n' {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }

                // Leer contenido hasta encontrar ``` DE CIERRE
                let mut content = String::new();
                let mut consecutive_backticks = 0;
                let mut found_closing = false;

                while self.pos < self.input.len() {
                    let c = self.current_char();
                    
                    if c == '`' {
                        consecutive_backticks += 1;
                        content.push(c);
                        self.advance();
                        self.column += 1;
                        
                        // Si tenemos 3 backticks consecutivos, verificamos si es cierre
                        if consecutive_backticks == 3 {
                            // Verificar que NO esté seguido de otro backtick (ej: ````)
                            if self.pos >= self.input.len() || self.current_char() != '`' {
                                // ¡Es el cierre! Remover los 3 backticks del contenido
                                content.pop(); // quitar tercer `
                                content.pop(); // quitar segundo `
                                content.pop(); // quitar primer `
                                found_closing = true;
                                break;
                            }
                        }
                    } else {
                        consecutive_backticks = 0; // Resetear contador
                        
                        if c == '\n' {
                            content.push('\n');
                            self.advance();
                            self.line += 1;
                            self.column = 1;
                        } else {
                            content.push(c);
                            self.advance();
                            self.column += 1;
                        }
                    }
                }

                if !found_closing {
                    eprintln!("⚠️ WARNING: CodeBlock sin cierre en línea {}. Se tomó hasta EOF.", start_line);
                }

                Token::new(
                    TokenType::CodeBlock { language: lang.clone(), content: content.clone() },
                    if lang.is_empty() { content } else { format!("{}|{}", lang, content) },
                    start_line,
                    start_col
                )
            } else {
                // Solo dos backticks, tratar como inline vacío
                Token::new(TokenType::CodeInline("".to_string()), "".to_string(), start_line, start_col)
            }
        } else {
            // Inline code: `código`
            let mut content = String::new();
            while self.pos < self.input.len() && self.current_char() != '`' {
                content.push(self.current_char());
                self.advance();
                self.column += 1;
            }

            if self.pos < self.input.len() {
                self.advance();
                self.column += 1;
            }

            Token::new(TokenType::CodeInline(content.clone()), content, start_line, start_col)
        }
    }

    fn tokenize_list_unordered(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;

        self.advance();
        self.column += 1;

        if self.pos < self.input.len() && self.current_char() == ' ' {
            self.advance();
            self.column += 1;
        }

        let mut content = String::new();
        while self.pos < self.input.len() && self.current_char() != '\n' {
            content.push(self.current_char());
            self.advance();
            self.column += 1;
        }

        Token::new(TokenType::ListUnordered, content, start_line, start_col)
    }

    fn try_tokenize_alert(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.column;
        
        if self.pos + 2 < self.input.len() 
            && self.current_char() == ':' 
            && self.input[self.pos + 1] == ':' 
            && self.input[self.pos + 2] == ':' 
        {
            self.advance();
            self.advance();
            self.advance();
            self.column += 3;
            
            let mut alert_type = String::new();
            while self.pos < self.input.len() 
                && self.current_char().is_alphanumeric() 
            {
                alert_type.push(self.current_char());
                self.advance();
                self.column += 1;
            }
            
            if !alert_type.is_empty() {
                return Token::new(
                    TokenType::AlertStart(alert_type.clone()), 
                    alert_type, 
                    start_line, 
                    start_col
                );
            } else {
                return Token::new(TokenType::AlertEnd, ":::".to_string(), start_line, start_col);
            }
        }
        
        let mut text = String::new();
        while self.pos < self.input.len() && self.current_char() != '\n' {
            text.push(self.current_char());
            self.advance();
            self.column += 1;
        }
        
        Token::new(TokenType::Paragraph, text, start_line, start_col)
    }

    fn current_char(&self) -> char {
        if self.pos < self.input.len() {
            self.input[self.pos]
        } else {
            '\0'
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            self.pos += 1;
        }
    }
}