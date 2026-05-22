# water - Compilador de Formato de Texto WebAssembly

> **[Read in English](README.md)** | **[Lire en français](README.fr.md)** | **[Ler em português](README.pt.md)**

Un compilador ligero y eficiente para el Formato de Texto WebAssembly (WAT), escrito en Rust.

## Descripción general

`water` es un analizador sintáctico y compilador mínimo pero eficiente para el formato de texto legible por humanos de WebAssembly. Proporciona una base para analizar módulos WAT, instrucciones, importaciones, funciones y otros componentes de WebAssembly.

## Características

- **Análisis rápido** - Análisis eficiente de instrucciones y módulos WAT usando `nom`
- **Dependencias mínimas** - Base de código ligera con solo las dependencias esenciales
- **Seguro en tipos** - Aprovecha el sistema de tipos de Rust para una representación segura del AST
- **Extensible** - Arquitectura modular compatible con emisores y transformaciones personalizados

## Estructura del proyecto

- `src/parser/` - Lógica de análisis WAT para instrucciones, funciones, importaciones y módulos
- `src/emitter/` - Utilidades de emisión y transformación de código
- `src/ast.rs` - Definiciones del Árbol de Sintaxis Abstracta (AST)
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

- **nom** (7.1.1) - Biblioteca de combinadores de analizadores sintácticos

## Licencia

Distribuido bajo la licencia MIT - consulta el archivo [LICENSE](LICENSE) para obtener más detalles.

## Estado del proyecto

Este es un proyecto experimental/educativo enfocado en comprender el análisis y la compilación del Formato de Texto WebAssembly.
