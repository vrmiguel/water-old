# water - WebAssembly Text Format Compiler

> **[Soma kwa Kiingereza](README.md)** | **[Soma kwa Kifaranga](README.fr.md)**

Kidogo lakini kicheza haraka compiler kwa WebAssembly Text Format (WAT), iliyoandikwa katika Rust.

## Muhtasari

`water` ni parser na compiler muhimu lakini mahususi kwa WebAssembly's inayosoma kwa kibinadamu muundo wa maandishi. Inatoa msingi kwa ajili ya kuchakata WAT modules, maagizo, imports, kazi, na sehemu nyingine za WebAssembly.

## Sifa

- **Kuchakata haraka** - Chukulizi cha haraka wa WAT maagizo na module kwa kutumia `nom`
- **Chache tu tegemezi** - Kod wa mwanga na tegemezi tu muhimu
- **Salama sana kwa aina** - Inatumia Rust's aina ya mfumo kwa AST salama
- **Inaweza kupanuliwa** - Usanidi wa moduli unaotumia emitters na mabadiliko ya kawaida

## Muundo wa Mradi

- `src/parser/` - Mantiki ya kuchakata WAT kwa maagizo, kazi, imports, na modules
- `src/emitter/` - Kutoa koodi na wasanidi wa mabadiliko
- `src/ast.rs` - Ufafanuzi wa Abstract Syntax Tree
- `src/leb128.rs` - LEB128 inayobadilika-urefu integer encoding
- `src/opcode.rs` - Ufafanuzi wa WebAssembly opcode

## Kuanza

### Mahitaji

- Rust 1.56 au zaidi

### Kujenga

```bash
cargo build --release
```

### Kuendesha

```bash
cargo run
```

## Tegemezi

- **nom** (7.1.1) - Parser combinators library

## Leseni

Leseni chini ya MIT License - angalia [LICENSE](LICENSE) faili kwa maelezo.

## Hali ya Mradi

Hii ni mradi wa jaribio/elimu unaozingatia kuelewa kwa kina WebAssembly Text Format kuchakata na compilation.
