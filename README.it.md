# water - Compilatore per il formato testuale di WebAssembly

> **[Read in English](README.md)** | **[Lire en français](README.fr.md)**

Un compilatore leggero e performante per il formato testuale di WebAssembly (WAT), scritto in Rust.

## Panoramica

`water` è un parser e compilatore minimale ma efficiente per il formato testuale leggibile di WebAssembly. Fornisce una base per l'analisi di moduli WAT, istruzioni, importazioni, funzioni e altri componenti di WebAssembly.

## Caratteristiche

- **Parsing veloce** - Analisi efficiente di istruzioni e moduli WAT utilizzando `nom`
- **Dipendenze minime** - Codebase leggero con solo le dipendenze essenziali
- **Type-safe** - Sfrutta il sistema di tipi di Rust per una rappresentazione sicura dell'AST
- **Estensibile** - Architettura modulare che supporta emettitori e trasformazioni personalizzati

## Struttura del progetto

- `src/parser/` - Logica di parsing WAT per istruzioni, funzioni, importazioni e moduli
- `src/emitter/` - Utilità per l'emissione e la trasformazione del codice
- `src/ast.rs` - Definizioni dell'Albero Sintattico Astratto (AST)
- `src/leb128.rs` - Codifica di interi a lunghezza variabile LEB128
- `src/opcode.rs` - Definizioni degli opcode di WebAssembly

## Per iniziare

### Prerequisiti

- Rust 1.56 o successivo

### Compilazione

```bash
cargo build --release
```

### Esecuzione

```bash
cargo run
```

## Dipendenze

- **nom** (7.1.1) - Libreria di combinatori di parser

## Licenza

Distribuito sotto la Licenza MIT - vedere il file [LICENSE](LICENSE) per i dettagli.

## Stato del progetto

Questo è un progetto sperimentale/educativo focalizzato sulla comprensione del parsing e della compilazione del formato testuale di WebAssembly.
