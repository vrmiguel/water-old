# water - Mkusanyaji wa Muundo wa Maandishi wa WebAssembly

> **[Read in English](README.md)** | **[Lire en français](README.fr.md)**

Mkusanyaji mwepesi na wenye utendaji bora kwa Muundo wa Maandishi wa WebAssembly (WAT), ulioandikwa kwa Rust.

## Muhtasari

`water` ni kichanganuzi na mkusanyaji mdogo lakini wenye ufanisi kwa muundo wa maandishi wa WebAssembly unaoweza kusomwa na binadamu. Hutoa msingi wa kuchanganua moduli za WAT, maagizo, uagizaji, vitendakazi, na vipengele vingine vya WebAssembly.

## Vipengele

- **Uchanganuzi wa haraka** - Uchanganuzi wenye ufanisi wa maagizo na moduli za WAT kwa kutumia `nom`
- **Utegemezi mdogo** - Msingi mwepesi wa msimbo wenye utegemezi muhimu pekee
- **Salama kwa aina** - Hutumia mfumo wa aina wa Rust kwa uwakilishi salama wa AST
- **Inayoweza kupanuliwa** - Usanifu wa moduli unaounga mkono visambazaji maalum na mabadiliko

## Muundo wa Mradi

- `src/parser/` - Mantiki ya uchanganuzi wa WAT kwa maagizo, vitendakazi, uagizaji, na moduli
- `src/emitter/` - Zana za usambazaji wa msimbo na mabadiliko
- `src/ast.rs` - Ufafanuzi wa Mti wa Sintaksia Dhahania (AST)
- `src/leb128.rs` - Usimbaji wa nambari kamili wa urefu unaobadilika wa LEB128
- `src/opcode.rs` - Ufafanuzi wa misimbo ya WebAssembly

## Kuanza

### Mahitaji ya Awali

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

- **nom** (7.1.1) - Maktaba ya viunganishi vya kichanganuzi

## Leseni

Imepewa leseni chini ya Leseni ya MIT - tazama faili la [LICENSE](LICENSE) kwa maelezo.

## Hali ya Mradi

Huu ni mradi wa majaribio/elimu unaolenga kuelewa uchanganuzi na ukusanyaji wa Muundo wa Maandishi wa WebAssembly.
