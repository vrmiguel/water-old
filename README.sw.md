# water - Kilinganiaji cha Muundo wa Matini wa WebAssembly

> **[Soma kwa Kifaranga](README.fr.md)**

Kilinganiaji nyepesi na kiendeleo cha Muundo wa Matini wa WebAssembly (WAT), iliyoandikwa kwa Rust.

## Muhtasari

`water` ni kigandaji na kilinganiaji nyepesi lakini wenye ufanisi kwa muundo wa matini unaosomakaushindi wa WebAssembly. Inatoa msingi wa kuchambua moduli za WAT, maagizo, uingizaji, chaguo za kukokotoa, na sehemu nyingine za WebAssembly.

## Sifa

- **Kugandua haraka** - Kuchambua kwa ufanisi maagizo na moduli za WAT kwa kutumia `nom`
- **Utegemezi mdogo** - Msingi wa msimbo nyepesi wenye utegemezi tu wa muhimu
- **Usalama wa aina** - Inatumia mfumo wa aina wa Rust kwa kuwakilisha AST salama
- **Inaweza kupanuliwa** - Usanifu wa modulari unaoambatanisha wastani wa custom na mabadiliko

## Muundo wa Mradi

- `src/parser/` - Mantiki ya kuchambua WAT kwa maagizo, chaguo za kukokotoa, uingizaji, na moduli
- `src/emitter/` - Zana za kutoleza na kubadilisha msimbo
- `src/ast.rs` - Ufafanuzi wa Mti wa Sintaksia ya Kujitokeza (AST)
- `src/leb128.rs` - Usimbaji wa nambari ya urefu wa LEB128
- `src/opcode.rs` - Ufafanuzi wa opcode za WebAssembly

## Kuanza

### Mahitaji Maalum

- Rust 1.56 au baadaye

### Kujenga

```bash
cargo build --release
```

### Kuendesha

```bash
cargo run
```

## Utegemezi

- **nom** (7.1.1) - Maktaba ya wachanganyaji wa kigandaji

## Leseni

Kulindwa chini ya Leseni ya MIT - angalia faili la [LICENSE](LICENSE) kwa maelezo zaidi.

## Hadhi ya Mradi

Hii ni mradi wa experimental/elimu unozingatia kuelewa kuchambua na kulingania muundo wa Matini wa WebAssembly.
