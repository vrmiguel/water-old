# water - Compilador de Formato de Texto WebAssembly

> **[Read in English](README.md)** | **[Lire en français](README.fr.md)** | **[Leer en español](README.es.md)** | **[Leggi in italiano](README.it.md)** | **[Auf Deutsch lesen](README.de.md)**

Um compilador leve e performático para o Formato de Texto WebAssembly (WAT), escrito em Rust.

## Visão Geral

`water` é um parser e compilador mínimo, mas eficiente, para o formato de texto legível por humanos do WebAssembly. Ele fornece uma base para analisar módulos WAT, instruções, importações, funções e outros componentes do WebAssembly.

## Recursos

- **Análise rápida** - Análise eficiente de instruções e módulos WAT usando `nom`
- **Dependências mínimas** - Base de código leve com apenas as dependências essenciais
- **Segurança de tipos** - Usa o sistema de tipos do Rust para uma representação segura da AST
- **Extensível** - Arquitetura modular com suporte a emissores e transformações personalizados

## Estrutura do Projeto

- `src/parser/` - Lógica de análise WAT para instruções, funções, importações e módulos
- `src/emitter/` - Utilitários de emissão e transformação de código
- `src/ast.rs` - Definições da Árvore Sintática Abstrata (AST)
- `src/leb128.rs` - Codificação de inteiros de tamanho variável LEB128
- `src/opcode.rs` - Definições de opcodes WebAssembly

## Começando

### Pré-requisitos

- Rust 1.56 ou posterior

### Compilação

```bash
cargo build --release
```

### Execução

```bash
cargo run
```

## Dependências

- **nom** (7.1.1) - Biblioteca de combinadores de parser

## Licença

Licenciado sob a Licença MIT - consulte o arquivo [LICENSE](LICENSE) para mais detalhes.

## Status do Projeto

Este é um projeto experimental/educacional focado em entender a análise e a compilação do Formato de Texto WebAssembly.
