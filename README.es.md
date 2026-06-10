# water - Compilador de WebAssembly Text Format

> **Traducciones:** [English](README.md) | [Français](README.fr.md)

Un compilador ligero y de alto rendimiento para WebAssembly Text Format (WAT), escrito en Rust.

## Descripción general

`water` es un parser y compilador mínimo pero eficiente para el formato de texto legible por humanos de WebAssembly. Proporciona una base para analizar módulos WAT, instrucciones, importaciones, funciones y otros componentes de WebAssembly.

## Características

- **Parsing rápido** - Parsing eficiente de instrucciones y módulos WAT usando `nom`
- **Dependencias mínimas** - Base de código ligera con solo las dependencias esenciales
- **Type-safe** - Aprovecha el sistema de tipos de Rust para una representación segura del AST
- **Extensible** - Arquitectura modular compatible con emisores y transformaciones personalizados

## Estructura del proyecto

- `src/parser/` - Lógica de parsing WAT para instrucciones, funciones, importaciones y módulos
- `src/emitter/` - Utilidades de emisión de código y transformación
- `src/ast.rs` - Definiciones del árbol de sintaxis abstracta (AST)
- `src/leb128.rs` - Codificación de enteros de longitud variable LEB128
- `src/opcode.rs` - Definiciones de opcodes de WebAssembly

## Primeros pasos

### Requisitos previos

- Rust 1.56 o posterior

### Compilación

```bash
cargo build --release
```

### Ejecución

```bash
cargo run
```

## Dependencias

- **nom** (7.1.1) - Biblioteca de combinadores de parsers

## Licencia

Licenciado bajo la licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más detalles.

## Estado del proyecto

Este es un proyecto experimental/educativo centrado en comprender el parsing y la compilación de WebAssembly Text Format.
