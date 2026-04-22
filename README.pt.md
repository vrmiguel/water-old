# water - Compilador do Formato de Texto WebAssembly

Um compilador leve e performático para o Formato de Texto WebAssembly (WAT), escrito em Rust.

## Visão Geral

`water` é um analisador sintático e compilador mínimo, porém eficiente, para o formato de texto legível por humanos do WebAssembly. Ele fornece uma base para analisar módulos WAT, instruções, imports, funções e outros componentes do WebAssembly.

## Funcionalidades

- **Análise rápida** - Análise eficiente de instruções e módulos WAT usando `nom`
- **Dependências mínimas** - Código-base leve com apenas as dependências essenciais
- **Tipagem segura** - Aproveita o sistema de tipos do Rust para uma representação segura da AST
- **Extensível** - Arquitetura modular com suporte a emissores e transformações personalizados

## Estrutura do Projeto

- `src/parser/` - Lógica de análise WAT para instruções, funções, imports e módulos
- `src/emitter/` - Utilitários de emissão e transformação de código
- `src/ast.rs` - Definições da Árvore Sintática Abstrata (AST)
- `src/leb128.rs` - Codificação de inteiros de tamanho variável LEB128
- `src/opcode.rs` - Definições dos opcodes do WebAssembly

## Primeiros Passos

### Pré-requisitos

- Rust 1.56 ou superior

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

Licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para mais detalhes.

## Status do Projeto

Este é um projeto experimental/educacional focado em entender a análise e compilação do Formato de Texto WebAssembly.
