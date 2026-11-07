# md-compiler 🦀

Un compilador de Markdown a HTML robusto y seguro, escrito completamente en **Rust**.

## 🚀 Demo Visual

![Demo Visual](https://github.com/user-attachments/assets/efe4891c-f7de-4ad6-9377-0de717ab2c7a)

## ✨ Características

- **Lexer Robusto**: Manejo correcto de saltos de línea Windows/Unix y caracteres especiales.
- **Parser Seguro**: Construcción de AST sin pánicos ni errores de borrowing.
- **Codegen Flexible**: Generación de HTML5 semántico con estilos CSS integrados.
- **Extensiones Custom**: Soporte nativo para alertas personalizadas (`:::info`, `:::warning`).
- **Seguridad XSS**: Escapado automático de caracteres HTML en el código generado.

## 🛠️ Cómo usarlo

    # Clonar el repositorio
    git clone https://github.com/jesuscreesponses9-arch/md-compiler.git
    cd md-compiler

    # Ejecutar el pipeline completo (Markdown -> HTML)
    cargo run --bin generate_html

    # Abrir el resultado
    start output.html  # Windows
    open output.html   # Mac/Linux

## 📂 Estructura del Proyecto

- `src/lexer/`: Análisis léxico
- `src/parser/`: Análisis sintáctico
- `src/codegen/`: Generación de código
- `src/bin/`: Binarios ejecutables
- `examples/`: Archivos de prueba

## 🤝 Contribuir

1. Haz un **Fork** del repositorio.
2. Crea una rama (`git checkout -b feature/nueva-funcionalidad`).
3. Realiza tus cambios y haz commit.
4. Push a la rama (`git push origin feature/nueva-funcionalidad`).
5. Abre un **Pull Request**.

## 📄 Licencia

Este proyecto está bajo la licencia MIT.
