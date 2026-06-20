# water - Compiler für das WebAssembly-Textformat

> **[Read in English](README.md)** | **[Lire en français](README.fr.md)** | **[Ler em português](README.pt-BR.md)** | **[Leer en español](README.es.md)** | **[Leggi in italiano](README.it.md)**

Ein leichtgewichtiger und performanter Compiler für das WebAssembly-Textformat (WAT), geschrieben in Rust.

## Überblick

`water` ist ein minimaler, aber effizienter Parser und Compiler für das menschenlesbare Textformat von WebAssembly. Es bietet eine Grundlage für das Parsen von WAT-Modulen, Anweisungen, Importen, Funktionen und anderen WebAssembly-Komponenten.

## Funktionen

- **Schnelles Parsen** - Effizientes Parsen von WAT-Anweisungen und -Modulen mit `nom`
- **Minimale Abhängigkeiten** - Leichtgewichtige Codebasis mit nur den wesentlichen Abhängigkeiten
- **Typsicher** - Nutzt Rusts Typsystem für eine sichere AST-Darstellung
- **Erweiterbar** - Modulare Architektur mit Unterstützung für eigene Emitter und Transformationen

## Projektstruktur

- `src/parser/` - WAT-Parsing-Logik für Anweisungen, Funktionen, Importe und Module
- `src/emitter/` - Hilfsprogramme für Codeausgabe und Transformationen
- `src/ast.rs` - Definitionen des abstrakten Syntaxbaums (AST)
- `src/leb128.rs` - LEB128-Codierung von Ganzzahlen variabler Länge
- `src/opcode.rs` - Definitionen der WebAssembly-Opcodes

## Erste Schritte

### Voraussetzungen

- Rust 1.56 oder neuer

### Build

```bash
cargo build --release
```

### Ausführen

```bash
cargo run
```

## Abhängigkeiten

- **nom** (7.1.1) - Bibliothek für Parser-Kombinatoren

## Lizenz

Lizenziert unter der MIT-Lizenz - siehe die Datei [LICENSE](LICENSE) für Details.

## Projektstatus

Dies ist ein experimentelles/lehrreiches Projekt, das sich auf das Verständnis des Parsens und Kompilierens des WebAssembly-Textformats konzentriert.
