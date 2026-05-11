# water - Compilatore per Formato Testo WebAssembly

Un compilatore leggero e performante per il Formato Testo WebAssembly (WAT), scritto in Rust.

## Panoramica

`water` è un parser e compilatore minimale ma efficiente per il formato testo leggibile dall'uomo di WebAssembly. Fornisce una base per l'analisi di moduli WAT, istruzioni, import, funzioni e altri componenti WebAssembly.

## Caratteristiche

- **Parsing veloce** - Analisi efficiente di istruzioni e moduli WAT tramite `nom`
- **Dipendenze minime** - Codebase leggero con solo le dipendenze essenziali
- **Type-safe** - Sfrutta il sistema di tipi di Rust per una rappresentazione sicura dell'AST
- **Estensibile** - Architettura modulare che supporta emitter e trasformazioni personalizzati

## Struttura del Progetto

- `src/parser/` - Logica di parsing WAT per istruzioni, funzioni, import e moduli
- `src/emitter/` - Utilità per l'emissione e la trasformazione del codice
- `src/ast.rs` - Definizioni dell'Albero Sintattico Astratto (AST)
- `src/leb128.rs` - Codifica di interi a lunghezza variabile LEB128
- `src/opcode.rs` - Definizioni degli opcode WebAssembly

## Per Iniziare

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

Distribuito sotto licenza MIT - vedi il file [LICENSE](LICENSE) per i dettagli.

## Stato del Progetto

Si tratta di un progetto sperimentale/educativo incentrato sulla comprensione del parsing e della compilazione del Formato Testo WebAssembly.
