# water - Compilatore per il Formato Testo di WebAssembly

Un compilatore leggero e performante per il formato testo di WebAssembly (WAT), scritto in Rust.

## Panoramica

`water` è un parser e compilatore minimale ma efficiente per il formato testo leggibile di WebAssembly. Fornisce le basi per il parsing di moduli WAT, istruzioni, import, funzioni e altri componenti di WebAssembly.

## Funzionalità

- **Parsing veloce** - Parsing efficiente delle istruzioni e dei moduli WAT tramite `nom`
- **Dipendenze minime** - Codebase leggera con solo le dipendenze essenziali
- **Sicurezza dei tipi** - Sfrutta il sistema di tipi di Rust per una rappresentazione sicura dell'AST
- **Estensibile** - Architettura modulare che supporta emettitori e trasformazioni personalizzate

## Struttura del Progetto

- `src/parser/` - Logica di parsing WAT per istruzioni, funzioni, import e moduli
- `src/emitter/` - Utilità per l'emissione e la trasformazione del codice
- `src/ast.rs` - Definizioni dell'Albero Sintattico Astratto (AST)
- `src/leb128.rs` - Codifica di interi a lunghezza variabile LEB128
- `src/opcode.rs` - Definizioni degli opcode di WebAssembly

## Per Iniziare

### Prerequisiti

- Rust 1.56 o versione successiva

### Compilazione

```bash
cargo build --release
```

### Esecuzione

```bash
cargo run
```

## Dipendenze

- **nom** (7.1.1) - Libreria di combinatori per il parsing

## Licenza

Distribuito sotto la licenza MIT - vedere il file [LICENSE](LICENSE) per i dettagli.

## Stato del Progetto

Questo è un progetto sperimentale/didattico focalizzato sulla comprensione del parsing e della compilazione del formato testo di WebAssembly.
