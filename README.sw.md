# water - Kikompila cha Muundo wa Maandishi wa WebAssembly

Kikompila chepesi na chenye utendaji mzuri kwa Muundo wa Maandishi wa WebAssembly (WAT), kilichoandikwa kwa Rust.

## Muhtasari

`water` ni kichanganuzi na kikompila kidogo lakini fanisi kwa muundo wa maandishi wa WebAssembly unaosomeka na binadamu. Hutoa msingi wa kuchanganua moduli za WAT, maagizo, imports, functions, na vipengele vingine vya WebAssembly.

## Vipengele

- **Uchanganuzaji wa haraka** - Uchanganuzaji fanisi wa maagizo na moduli za WAT kwa kutumia `nom`
- **Utegemezi mdogo** - Msingi wa msimbo mwepesi wenye utegemezi muhimu pekee
- **Usalama wa aina** - Hutumia mfumo wa aina wa Rust kwa uwakilishi salama wa AST
- **Rahisi kupanua** - Usanifu wa kimoduli unaowezesha emitters na transformations maalum

## Muundo wa Mradi

- `src/parser/` - Mantiki ya uchanganuzaji wa WAT kwa maagizo, functions, imports, na moduli
- `src/emitter/` - Zana za utoaji wa msimbo na transformations
- `src/ast.rs` - Ufafanuzi wa Abstract Syntax Tree (AST)
- `src/leb128.rs` - Usimbaji wa namba nzima za urefu unaobadilika wa LEB128
- `src/opcode.rs` - Ufafanuzi wa WebAssembly opcodes

## Kuanza

### Mahitaji

- Rust 1.56 au toleo jipya zaidi

### Kujenga

```bash
cargo build --release
```

### Kuendesha

```bash
cargo run
```

## Utegemezi

- **nom** (7.1.1) - Maktaba ya parser combinators

## Leseni

Imetolewa chini ya Leseni ya MIT - tazama faili [LICENSE](LICENSE) kwa maelezo zaidi.

## Hali ya Mradi

Huu ni mradi wa majaribio/kielimu unaolenga kuelewa uchanganuzaji na ukompilishaji wa Muundo wa Maandishi wa WebAssembly.
