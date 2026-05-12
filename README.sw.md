# water - Mkutani wa Muundo wa Maandishi wa WebAssembly

> **[Kusoma kwa Kiswahili](README.sw.md)**

Mkutani mwepesi na wenye nguvu kwa Muundo wa Maandishi wa WebAssembly (WAT), umeandikwa katika Rust.

## Muhtasari

`water` ni mkutani na mkusanyaji mdogo lakini wenye tija kwa muundo wa maandishi wa WebAssembly unaosomeka katika lugha ya binadamu. Inatoa msingi wa kuchanganua moduli WAT, maagizo, uagizaji, kazi, na sehemu nyingine za WebAssembly.

## Sifa

- **Uchambuzi wa Haraka** - Uchambuzi wenye tija wa maagizo na moduli WAT kwa kutumia `nom`
- **Tegemezi Ndogo** - Msimbo mwepesi na tegemezi tu ya muhimu
- **Salama kwa Aina** - Inatumia mfumo wa aina wa Rust kwa ajili ya uwakilishi salama wa AST
- **Panuka** - Usanifu wa moduli unaounga mkakati wa wakati na mabadiliko ya kawaida

## Muundo wa Mraba

- `src/parser/` - Mantiki ya uchambuzi WAT kwa maagizo, kazi, uagizaji, na moduli
- `src/emitter/` - Zana za kutoleza msimbo na mabadiliko
- `src/ast.rs` - Ufafanuzi wa Miti ya Sintaksi Halisi (AST)
- `src/leb128.rs` - Ukamataji wa namba wenye urefu unaobadilika LEB128
- `src/opcode.rs` - Ufafanuzi wa msimbo wa operesheni wa WebAssembly

## Kuanza

### Mahitaji

- Rust 1.56 au baadaye

### Kujenga

```bash
cargo build --release
```

### Kuendesha

```bash
cargo run
```

## Tegemezi

- **nom** (7.1.1) - Kitabu cha wafanyakazi wa kuchanganua sintaksi

## Leseni

Leseni chini ya MIT - tazama faili la [LICENSE](LICENSE) kwa maelezo zaidi.

## Hali ya Mraba

Huu ni mraba wa majaribio/elimu unaozingatia uelewa wa uchambuzi na ukamataji wa Muundo wa Maandishi wa WebAssembly.
