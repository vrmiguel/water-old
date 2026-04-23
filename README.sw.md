# water - Mkusanyaji wa Muundo wa Maandishi wa WebAssembly

Mkusanyaji mwepesi na wenye utendaji bora kwa Muundo wa Maandishi wa WebAssembly (WAT), ulioandikwa kwa Rust.

## Muhtasari

`water` ni kichanganuzi na mkusanyaji mdogo lakini wenye ufanisi kwa muundo wa maandishi wa WebAssembly unaosomeka na binadamu. Unatoa msingi wa kuchanganua moduli za WAT, maagizo, uagizaji, kazi, na vipengele vingine vya WebAssembly.

## Sifa

- **Uchanganuzi wa haraka** - Uchanganuzi wenye ufanisi wa maagizo na moduli za WAT kwa kutumia `nom`
- **Utegemezi mdogo** - Msimbo mwepesi wenye utegemezi wa lazima tu
- **Salama kwa aina** - Unatumia mfumo wa aina wa Rust kwa uwakilishi salama wa AST
- **Unaweza kupanuliwa** - Muundo wa moduli unaounga mkono watoaji na mabadiliko maalum

## Muundo wa Mradi

- `src/parser/` - Mantiki ya uchanganuzi wa WAT kwa maagizo, kazi, uagizaji, na moduli
- `src/emitter/` - Zana za utoaji na ubadilishaji wa msimbo
- `src/ast.rs` - Ufafanuzi wa Mti wa Sintaksia Uliodhahanishwa (AST)
- `src/leb128.rs` - Usimbaji wa nambari kamili wa urefu tofauti wa LEB128
- `src/opcode.rs` - Ufafanuzi wa opcode za WebAssembly

## Kuanza

### Mahitaji ya awali

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

- **nom** (7.1.1) - Maktaba ya vichanganuzi vya pamoja (parser combinators)

## Leseni

Imesambazwa chini ya Leseni ya MIT - angalia faili ya [LICENSE](LICENSE) kwa maelezo zaidi.

## Hali ya Mradi

Huu ni mradi wa majaribio/elimu unaolenga kuelewa uchanganuzi na ukusanyaji wa Muundo wa Maandishi wa WebAssembly.
