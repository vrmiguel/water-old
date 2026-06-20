# water - Compilatore di Formato Testuale WebAssembly

> **[Read in English](README.md)** | **[Lire en français](README.fr.md)** | **[Ler em português](README.pt-BR.md)** | **[Leer en español](README.es.md)** | **[Auf Deutsch lesen](README.de.md)**

Un compilatore leggero e performante per il Formato Testuale WebAssembly (WAT), scritto in Rust.

## Panoramica

`water` è un parser e compilatore minimale ma efficiente per il formato testuale leggibile dagli esseri umani di WebAssembly. Fornisce una base per analizzare moduli WAT, istruzioni, importazioni, funzioni e altri componenti WebAssembly.

## Funzionalità

- **Analisi rapida** - Analisi efficiente di istruzioni e moduli WAT usando `nom`
- **Dipendenze minime** - Base di codice leggera con solo le dipendenze essenziali
- **Sicurezza dei tipi** - Sfrutta il sistema di tipi di Rust per una rappresentazione sicura dell'AST
- **Estensibile** - Architettura modulare con supporto per emettitori e trasformazioni personalizzati

## Struttura del Progetto

- `src/parser/` - Logica di analisi WAT per istruzioni, funzioni, importazioni e moduli
- `src/emitter/` - Utilità di emissione e trasformazione del codice
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

Distribuito sotto la Licenza MIT - consulta il file [LICENSE](LICENSE) per maggiori dettagli.

## Stato del Progetto

Questo è un progetto sperimentale/educativo focalizzato sulla comprensione dell'analisi e della compilazione del Formato Testuale WebAssembly.
