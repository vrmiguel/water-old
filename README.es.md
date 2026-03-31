# water - Compilador de Formato de Texto WebAssembly

Un compilador ligero y de alto rendimiento para el Formato de Texto de WebAssembly (WAT), escrito en Rust.

## Descripción General

`water` es un analizador y compilador mínimo pero eficiente para el formato de texto legible de WebAssembly. Proporciona una base para analizar módulos WAT, instrucciones, importaciones, funciones y otros componentes de WebAssembly.

## Características

- **Análisis rápido** - Análisis eficiente de instrucciones y módulos WAT usando `nom`
- **Dependencias mínimas** - Base de código ligera con solo las dependencias esenciales
- **Seguridad de tipos** - Aprovecha el sistema de tipos de Rust para una representación segura del AST
- **Extensible** - Arquitectura modular que soporta emisores y transformaciones personalizadas

## Estructura del Proyecto

- `src/parser/` - Lógica de análisis WAT para instrucciones, funciones, importaciones y módulos
- `src/emitter/` - Utilidades de emisión y transformación de código
- `src/ast.rs` - Definiciones del Árbol de Sintaxis Abstracta
- `src/leb128.rs` - Codificación de enteros de longitud variable LEB128
- `src/opcode.rs` - Definiciones de opcodes de WebAssembly

## Primeros Pasos

### Requisitos Previos

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

- **nom** (7.1.1) - Biblioteca de combinadores de analizadores

## Licencia

Licenciado bajo la Licencia MIT - consulte el archivo [LICENSE](LICENSE) para más detalles.

## Estado del Proyecto

Este es un proyecto experimental/educativo enfocado en comprender el análisis y la compilación del Formato de Texto de WebAssembly.
