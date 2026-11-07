use md_compiler::lexer::Tokenizer;
use md_compiler::parser::Parser;
use md_compiler::codegen::generate_html;
use std::fs;

fn main() {
    // 1. Leer el archivo Markdown
    let content = fs::read_to_string("examples/test.md")
        .expect("❌ Error: No se pudo leer examples/test.md");

    println!("📄 Leyendo: examples/test.md");

    // 2. Fase Lexer: Markdown → Tokens
    let mut tokenizer = Tokenizer::new(&content);
    let tokens = tokenizer.tokenize();
    println!("   ✅ {} tokens generados", tokens.len());

    // 3. Fase Parser: Tokens → AST
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("   ✅ {} nodos AST construidos", ast.len());

    // 4. Fase Codegen: AST → HTML
    let html = generate_html(&ast);
    
    // 5. Guardar en archivo
    let output_path = "output.html";
    fs::write(output_path, &html).expect("❌ Error: No se pudo escribir output.html");
    
    println!("   ✅ HTML generado: {}", output_path);
    println!("\n🎉 ¡Pipeline completo exitoso!");
    println!("   Abre '{}' en tu navegador para ver el resultado.", output_path);
}