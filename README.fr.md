# water - Compilateur de Format Texte WebAssembly

> **[Read in English](README.md)**

Un compilateur léger et performant pour le format texte WebAssembly (WAT), écrit en Rust.

## Aperçu

`water` est un analyseur syntaxique et compilateur minimal mais efficace pour le format texte lisible par l'humain de WebAssembly. Il fournit une base pour analyser les modules WAT, les instructions, les imports, les fonctions et d'autres composants WebAssembly.

## Fonctionnalités

- **Analyse rapide** - Analyse efficace des instructions et des modules WAT avec `nom`
- **Dépendances minimales** - Base de code légère avec uniquement les dépendances essentielles
- **Typage sûr** - Exploite le système de types de Rust pour représenter l'AST en toute sécurité
- **Extensible** - Architecture modulaire prenant en charge des émetteurs et transformations personnalisés

## Structure du projet

- `src/parser/` - Logique d'analyse WAT pour les instructions, fonctions, imports et modules
- `src/emitter/` - Utilitaires d'émission et de transformation de code
- `src/ast.rs` - Définitions de l'arbre syntaxique abstrait (AST)
- `src/leb128.rs` - Encodage d'entiers à longueur variable LEB128
- `src/opcode.rs` - Définitions des opcodes WebAssembly

## Bien démarrer

### Prérequis

- Rust 1.56 ou ultérieur

### Compilation

```bash
cargo build --release
```

### Exécution

```bash
cargo run
```

## Dépendances

- **nom** (7.1.1) - Bibliothèque de combinateurs d'analyseurs syntaxiques

## Licence

Sous licence MIT - consultez le fichier [LICENSE](LICENSE) pour plus de détails.

## État du projet

Il s'agit d'un projet expérimental et éducatif axé sur la compréhension de l'analyse et de la compilation du format texte WebAssembly.
