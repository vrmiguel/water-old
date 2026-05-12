# water - Mkutano wa WebAssembly Text Format

> **[Lire en français](README.fr.md) | [Read in English](README.md)**

Mkutanaji wa WebAssembly Text Format (WAT) ulio nyepesi na wenye utendaji mzuri, umefanywa kwa Rust.

## Muhtasari

`water` ni mkutanaji wa chini lakini yenye ufanisi wa kuzamili na kuandaa umbizo la maandishi linaloweza kusoma la WebAssembly. Inatoa msingi wa kuzamili moduli za WAT, maelekezo, uagizaji, kazi, na sehemu nyingine za WebAssembly.

## Vipengele

- **Kuzamili haraka** - Kuzamili kwa ufanisi kwa WAT maelekezo na moduli kwa kutumia `nom`
- **Tegemezi chache** - Misimbo nyepesi yenye tegemezi vya lazima tu
- **Salama kwa aina** - Inatumia mfumo wa aina wa Rust kwa uwakilishi salama wa AST
- **Inaweza kupanuliwa** - Sanaa ya moduli inayosaidia kuzamili kwa kaida na mabadiliko

## Muundo wa Mradi

- `src/parser/` - Mantiki ya kuzamili WAT kwa maelekezo, kazi, uagizaji, na moduli
- `src/emitter/` - Usambazaji wa kanuni na zana za mabadiliko
- `src/ast.rs` - Ufafanuzi wa Abstract Syntax Tree
- `src/leb128.rs` - Kusimba kwa nambari kamili za urefu tofauti wa LEB128
- `src/opcode.rs` - Ufafanuzi wa WebAssembly opcode

## Kuanza

### Mahitaji Mapema

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

- **nom** (7.1.1) - Maktaba ya mkutanaji wa mchanganyiko

## Leseni

Kumilikiwa chini ya Leseni ya MIT - tazama faili ya [LICENSE](LICENSE) kwa maelezo.

## Hali ya Mradi

Hii ni mradi wa majaribio/elimu unaozingatia kuelewa kuzamili umbizo la maandishi wa WebAssembly na ukutaji.
