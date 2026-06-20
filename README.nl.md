# water - Compiler voor WebAssembly-tekstformaat

> **[Read in English](README.md)** | **[Lire en français](README.fr.md)** | **[Ler em português](README.pt-BR.md)** | **[Leer en español](README.es.md)** | **[Leggi in italiano](README.it.md)** | **[Auf Deutsch lesen](README.de.md)**

Een lichte en performante compiler voor het WebAssembly-tekstformaat (WAT), geschreven in Rust.

## Overzicht

`water` is een minimale maar efficiënte parser en compiler voor het menselijk leesbare tekstformaat van WebAssembly. Het biedt een basis voor het parsen van WAT-modules, instructies, imports, functies en andere WebAssembly-componenten.

## Functies

- **Snel parsen** - Efficiënt parsen van WAT-instructies en -modules met `nom`
- **Minimale afhankelijkheden** - Lichte codebase met alleen de essentiële afhankelijkheden
- **Typeveilig** - Gebruikt Rusts typesysteem voor een veilige AST-representatie
- **Uitbreidbaar** - Modulaire architectuur met ondersteuning voor aangepaste emitters en transformaties

## Projectstructuur

- `src/parser/` - WAT-parsinglogica voor instructies, functies, imports en modules
- `src/emitter/` - Hulpmiddelen voor code-emissie en transformaties
- `src/ast.rs` - Definities van de Abstract Syntax Tree (AST)
- `src/leb128.rs` - LEB128-codering voor gehele getallen met variabele lengte
- `src/opcode.rs` - Definities van WebAssembly-opcodes

## Aan de Slag

### Vereisten

- Rust 1.56 of nieuwer

### Bouwen

```bash
cargo build --release
```

### Uitvoeren

```bash
cargo run
```

## Afhankelijkheden

- **nom** (7.1.1) - Bibliotheek voor parsercombinatoren

## Licentie

Gelicentieerd onder de MIT-licentie - zie het bestand [LICENSE](LICENSE) voor details.

## Projectstatus

Dit is een experimenteel/educatief project gericht op het begrijpen van het parsen en compileren van het WebAssembly-tekstformaat.
