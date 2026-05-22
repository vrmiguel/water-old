# water - Compilador do Formato de Texto WebAssembly

> **[Read in English](README.md)** | **[Lire en français](README.fr.md)** | **[Leer en español](README.es.md)**

Um compilador leve e eficiente para o Formato de Texto WebAssembly (WAT), escrito em Rust.

## Visão geral

`water` é um analisador sintático e compilador mínimo, mas eficiente, para o formato de texto legível por humanos do WebAssembly. Ele fornece uma base para analisar módulos WAT, instruções, importações, funções e outros componentes do WebAssembly.

## Recursos

- **Análise rápida** - Análise eficiente de instruções e módulos WAT usando `nom`
- **Dependências mínimas** - Base de código leve com apenas as dependências essenciais
- **Seguro em tipos** - Aproveita o sistema de tipos do Rust para uma representação segura da AST
- **Extensível** - Arquitetura modular compatível com emissores e transformações personalizados

## Estrutura do projeto

- `src/parser/` - Lógica de análise WAT para instruções, funções, importações e módulos
- `src/emitter/` - Utilitários de emissão e transformação de código
- `src/ast.rs` - Definições da Árvore de Sintaxe Abstrata (AST)
- `src/leb128.rs` - Codificação de inteiros de comprimento variável LEB128
- `src/opcode.rs` - Definições de opcodes do WebAssembly

## Primeiros passos

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

- **nom** (7.1.1) - Biblioteca de combinadores de analisadores sintáticos

## Licença

Licenciado sob a licença MIT - consulte o arquivo [LICENSE](LICENSE) para obter mais detalhes.

## Estado do projeto

Este é um projeto experimental/educacional focado em compreender a análise e a compilação do Formato de Texto WebAssembly.
