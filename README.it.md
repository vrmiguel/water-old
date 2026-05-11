# water - Compilatore del Formato Testo WebAssembly

Un compilatore leggero e performante per il Formato Testo WebAssembly (WAT), scritto in Rust.

## Panoramica

`water` è un parser e compilatore minimale ma efficiente per il formato testo leggibile di WebAssembly. Fornisce una base per l'analisi di moduli WAT, istruzioni, import, funzioni e altri componenti WebAssembly.

## Caratteristiche

- **Analisi veloce** - Analisi efficiente di istruzioni e moduli WAT tramite `nom`
- **Dipendenze minime** - Base di codice leggera con solo le dipendenze essenziali
- **Type-safe** - Sfrutta il sistema di tipi di Rust per una rappresentazione sicura dell'AST
- **Estensibile** - Architettura modulare che supporta emettitori e trasformazioni personalizzati

## Struttura del Progetto

- `src/parser/` - Logica di analisi WAT per istruzioni, funzioni, import e moduli
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

Distribuito con licenza MIT - vedere il file [LICENSE](LICENSE) per i dettagli.

## Stato del Progetto

Si tratta di un progetto sperimentale/educativo focalizzato sulla comprensione dell'analisi e della compilazione del Formato Testo WebAssembly.
