# water - Compilatore per WebAssembly Text Format

> **Traduzioni:** [English](README.md) | [Français](README.fr.md) | [日本語](README.ja.md)

Un compilatore leggero e performante per WebAssembly Text Format (WAT), scritto in Rust.

## Panoramica

`water` è un parser e compilatore minimale ma efficiente per il formato testuale leggibile dall'uomo di WebAssembly. Fornisce una base per analizzare moduli WAT, istruzioni, import, funzioni e altri componenti WebAssembly.

## Funzionalità

- **Parsing veloce** - Parsing efficiente di istruzioni e moduli WAT usando `nom`
- **Dipendenze minime** - Codebase leggera con solo le dipendenze essenziali
- **Type-safe** - Sfrutta il sistema di tipi di Rust per una rappresentazione sicura dell'AST
- **Estensibile** - Architettura modulare che supporta emitter e trasformazioni personalizzati

## Struttura del progetto

- `src/parser/` - Logica di parsing WAT per istruzioni, funzioni, import e moduli
- `src/emitter/` - Utilità di emissione del codice e trasformazione
- `src/ast.rs` - Definizioni dell'Abstract Syntax Tree (AST)
- `src/leb128.rs` - Codifica di interi a lunghezza variabile LEB128
- `src/opcode.rs` - Definizioni degli opcode WebAssembly

## Per iniziare

### Prerequisiti

- Rust 1.56 o versioni successive

### Compilazione

```bash
cargo build --release
```

### Esecuzione

```bash
cargo run
```

## Dipendenze

- **nom** (7.1.1) - Libreria di parser combinator

## Licenza

Distribuito sotto licenza MIT - consulta il file [LICENSE](LICENSE) per i dettagli.

## Stato del progetto

Questo è un progetto sperimentale/educativo incentrato sulla comprensione del parsing e della compilazione di WebAssembly Text Format.
