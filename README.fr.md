# water - Compilateur de Format Texte WebAssembly

Un compilateur léger et performant pour le Format Texte de WebAssembly (WAT), écrit en Rust.

## Aperçu

`water` est un analyseur et compilateur minimal mais efficace pour le format texte lisible de WebAssembly. Il fournit une base pour l'analyse des modules WAT, des instructions, des importations, des fonctions et d'autres composants WebAssembly.

## Fonctionnalités

- **Analyse rapide** - Analyse efficace des instructions et des modules WAT avec `nom`
- **Dépendances minimales** - Base de code légère avec uniquement les dépendances essentielles
- **Typage sûr** - Exploite le système de types de Rust pour une représentation sûre de l'AST
- **Extensible** - Architecture modulaire prenant en charge des émetteurs et des transformations personnalisés

## Structure du Projet

- `src/parser/` - Logique d'analyse WAT pour les instructions, fonctions, importations et modules
- `src/emitter/` - Utilitaires d'émission et de transformation de code
- `src/ast.rs` - Définitions de l'Arbre Syntaxique Abstrait
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

- **nom** (7.1.1) - Bibliothèque de combinateurs d'analyseurs

## Licence

Distribué sous la licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## État du Projet

Il s'agit d'un projet expérimental/éducatif axé sur la compréhension de l'analyse et de la compilation du Format Texte de WebAssembly.
