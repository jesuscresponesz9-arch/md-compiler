use std::fs;

fn main() {
    // Construimos el contenido línea por línea para evitar problemas con backticks en raw strings
    let lines = vec![
        "# Bienvenido al Compilador",
        "",
        "Este es un **párrafo** con texto en *cursiva* y `código inline`.",
        "",
        "## Lista de características",
        "",
        "- Tokenización robusta",
        "- Análisis sintáctico seguro",
        "- Generación de código HTML",
        "",
        "```rust",
        "fn main() {",
        "    println!(\"¡Hola desde Rust!\");",
        "}",
        "```",
        "",
        ":::info",
        "Esta es una alerta personalizada con contenido multilinea.",
        "Puede tener varios párrafos internos.",
        ":::",
    ];

    let content = lines.join("\n") + "\n";

    fs::write("examples/test.md", content).expect("Error al escribir test.md");
    println!("✅ examples/test.md generado correctamente con Rust puro 🦀");
}