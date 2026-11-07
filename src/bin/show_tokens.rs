use md_compiler::lexer::Tokenizer;
use std::fs;

fn main() {
    // Leer el archivo de ejemplo
    let content = fs::read_to_string("examples/test.md")
        .expect("Error: No se pudo leer examples/test.md");

    println!("📄 Leyendo: examples/test.md\n");
    println!("═══════════════════════════════════════");

    // Tokenizar
    let mut tokenizer = Tokenizer::new(&content);
    let tokens = tokenizer.tokenize();

    // Mostrar cada token
    for (i, token) in tokens.iter().enumerate() {
        println!(
            "[{:>3}] {:<20} | Línea {:>3}, Col {:>3} | Valor: \"{}\"",
            i,
            format!("{:?}", token.token_type),
            token.line,
            token.column,
            token.value.replace('\n', "\\n")
        );
    }

    println!("═══════════════════════════════════════");
    println!("✅ Total: {} tokens generados", tokens.len());
}