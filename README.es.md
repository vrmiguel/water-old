# water - Compilador del formato de texto de WebAssembly

> **Idiomas: [Français](README.fr.md) | Español**

Un compilador ligero y eficiente para el formato de texto de WebAssembly (WAT), escrito en Rust.

## Descripción general

`water` es un analizador sintáctico y un compilador minimalista, pero eficiente, para el formato de texto de WebAssembly, legible por humanos. Proporciona una base para analizar módulos WAT, instrucciones, importaciones, funciones y otros componentes de WebAssembly.

## Características

- **Análisis rápido** - Análisis eficiente de instrucciones y módulos WAT mediante `nom`
- **Dependencias mínimas** - Base de código ligera con solo las dependencias esenciales
- **Tipado seguro** - Aprovecha el sistema de tipos de Rust para una representación segura del AST
- **Extensible** - Arquitectura modular compatible con emisores y transformaciones personalizados

## Estructura del proyecto

- `src/parser/` - Lógica de análisis WAT para instrucciones, funciones, importaciones y módulos
- `src/emitter/` - Utilidades de emisión y transformación de código
- `src/ast.rs` - Definiciones del árbol de sintaxis abstracta (AST)
- `src/leb128.rs` - Codificación LEB128 de enteros de longitud variable
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

Distribuido bajo la licencia MIT; consulta el archivo [LICENSE](LICENSE) para obtener más detalles.

## Estado del proyecto

Este es un proyecto experimental y educativo centrado en comprender el análisis y la compilación del formato de texto de WebAssembly.
