# water - WebAssembly Text Format コンパイラ

> **翻訳:** [English](README.md) | [Français](README.fr.md) | [Italiano](README.it.md)

Rust で書かれた、軽量で高性能な WebAssembly Text Format (WAT) コンパイラです。

## 概要

`water` は、WebAssembly の人間が読めるテキスト形式を扱う、最小限でありながら効率的なパーサーおよびコンパイラです。WAT モジュール、命令、インポート、関数、その他の WebAssembly コンポーネントを解析するための基盤を提供します。

## 機能

- **高速な解析** - `nom` を使用した効率的な WAT 命令およびモジュール解析
- **最小限の依存関係** - 必要不可欠な依存関係のみを持つ軽量なコードベース
- **型安全** - Rust の型システムを活用した安全な AST 表現
- **拡張可能** - カスタムエミッターや変換をサポートするモジュール化されたアーキテクチャ

## プロジェクト構成

- `src/parser/` - 命令、関数、インポート、モジュール向けの WAT 解析ロジック
- `src/emitter/` - コード生成および変換ユーティリティ
- `src/ast.rs` - 抽象構文木 (AST) の定義
- `src/leb128.rs` - LEB128 可変長整数エンコーディング
- `src/opcode.rs` - WebAssembly オペコード定義

## はじめに

### 前提条件

- Rust 1.56 以降

### ビルド

```bash
cargo build --release
```

### 実行

```bash
cargo run
```

## 依存関係

- **nom** (7.1.1) - パーサーコンビネーターライブラリ

## ライセンス

MIT ライセンスの下でライセンスされています。詳細は [LICENSE](LICENSE) ファイルを参照してください。

## プロジェクトの状態

これは WebAssembly Text Format の解析とコンパイルを理解することに焦点を当てた、実験的/教育的なプロジェクトです。
