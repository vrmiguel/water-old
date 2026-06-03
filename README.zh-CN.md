# water - WebAssembly 文本格式编译器

> **[Read in English](README.md)** | **[Lire en français](README.fr.md)**

一个用 Rust 编写的轻量且高性能的 WebAssembly 文本格式（WAT）编译器。

## 概述

`water` 是一个用于 WebAssembly 人类可读文本格式的极简且高效的解析器和编译器。它为解析 WAT 模块、指令、导入、函数以及其他 WebAssembly 组件提供了基础。

## 特性

- **快速解析** - 使用 `nom` 高效解析 WAT 指令和模块
- **最小依赖** - 轻量级代码库，仅包含必要依赖
- **类型安全** - 利用 Rust 的类型系统安全地表示 AST
- **可扩展** - 模块化架构支持自定义发射器和转换

## 项目结构

- `src/parser/` - 用于指令、函数、导入和模块的 WAT 解析逻辑
- `src/emitter/` - 代码发射和转换工具
- `src/ast.rs` - 抽象语法树（AST）定义
- `src/leb128.rs` - LEB128 可变长度整数编码
- `src/opcode.rs` - WebAssembly 操作码定义

## 快速开始

### 前置要求

- Rust 1.56 或更高版本

### 构建

```bash
cargo build --release
```

### 运行

```bash
cargo run
```

## 依赖

- **nom** (7.1.1) - 解析器组合子库

## 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。

## 项目状态

这是一个实验性/教育性项目，重点在于理解 WebAssembly 文本格式的解析和编译。
