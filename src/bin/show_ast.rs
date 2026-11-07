use md_compiler::lexer::Tokenizer;
use md_compiler::parser::Parser;
use std::fs;

fn main() {
    let content = fs::read_to_string("examples/test.md")
        .expect("Error: No se pudo leer examples/test.md");

    println!("📄 Leyendo: examples/test.md\n");
    println!("═══════════════════════════════════════");
    println!("🌳 ÁRBOL DE SINTAXIS ABSTRACTA (AST)");
    println!("═══════════════════════════════════════\n");

    // Fase 1: Tokenizar
    let mut tokenizer = Tokenizer::new(&content);
    let tokens = tokenizer.tokenize();
    
    // DEBUG: Mostrar tokens para verificar si la alerta está ahí
    println!("🔍 Tokens generados: {}", tokens.len());
    for (i, t) in tokens.iter().enumerate() {
        println!("   [{}] {:?}", i, t.token_type);
    }
    println!();

    // Fase 2: Parsear
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Mostrar AST
    for (i, node) in ast.iter().enumerate() {
        match node {
            md_compiler::parser::AstNode::Heading { level, content } => {
                let hashes = "#".repeat(*level as usize);
                println!("[{:>2}] {} {}", i, hashes, content);
            }
            
            md_compiler::parser::AstNode::Paragraph { content } => {
                print!("[{:>2}] 📝 Párrafo: ", i);
                for inline in content {
                    match inline {
                        md_compiler::parser::InlineNode::Text(t) => print!("{}", t),
                        md_compiler::parser::InlineNode::Bold(t) => print!("**{}**", t),
                        md_compiler::parser::InlineNode::Italic(t) => print!("*{}*", t),
                        md_compiler::parser::InlineNode::CodeInline(t) => print!("`{}`", t),
                    }
                }
                println!();
            }
            
            md_compiler::parser::AstNode::ListUnordered { items } => {
                println!("[{:>2}] 📋 Lista:", i);
                for item in items {
                    println!("      • {}", item);
                }
            }
            
            // CORRECCIÓN: Formato limpio para CodeBlock
            md_compiler::parser::AstNode::CodeBlock { language, code } => {
                let lang = language.as_deref().unwrap_or("plain");
                println!("[{:>2}] 💻 CodeBlock [{}]:", i, lang);
                println!("      ```");
                for line in code.lines() {
                    println!("      {}", line);
                }
                println!("      ```");
            }
            
            md_compiler::parser::AstNode::Alert { alert_type, content } => {
                println!("[{:>2}] ⚠️  Alerta [{}]:", i, alert_type);
                println!("      {}", content);
            }
            
            md_compiler::parser::AstNode::BlankLine => {
                println!("[{:>2}] ↕️  (espaciado)", i);
            }
        }
    }

    println!("\n═══════════════════════════════════════");
    println!("✅ Total: {} nodos en el AST", ast.len());
}