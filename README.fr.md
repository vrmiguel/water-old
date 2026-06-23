# water - Compilateur de Format Texte WebAssembly

Un compilateur léger et performant pour le Format Texte WebAssembly (WAT), écrit en Rust et maintenu comme projet WIP précoce.

## Aperçu

`water` est un analyseur syntaxique et compilateur minimal mais efficace pour le format texte lisible par l'homme de WebAssembly. Il fournit une base pour l'analyse des modules WAT, des instructions, des imports, des fonctions et d'autres composants WebAssembly.

## Fonctionnalités

- **Analyse rapide** - Analyse efficace des instructions et modules WAT grâce à `nom`
- **Dépendances minimales** - Base de code légère avec uniquement les dépendances essentielles
- **Typage sûr** - Exploite le système de types de Rust pour une représentation sûre de l'AST
- **Extensible** - Architecture modulaire prenant en charge des émetteurs et transformations personnalisés

## Structure du Projet

- `src/parser/` - Logique d'analyse WAT pour les instructions, fonctions, imports et modules
- `src/emitter/` - Utilitaires d'émission et de transformation de code
- `src/ast.rs` - Définitions de l'Arbre Syntaxique Abstrait (AST)
- `src/leb128.rs` - Encodage d'entiers à longueur variable LEB128
- `src/opcode.rs` - Définitions des opcodes WebAssembly

## Pour Commencer

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

Distribué sous la licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## État du Projet

Il s'agit d'un projet expérimental/éducatif axé sur la compréhension de l'analyse et de la compilation du Format Texte WebAssembly.
