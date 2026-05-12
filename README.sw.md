# water - Mtungaji wa Muundo wa Maandishi wa WebAssembly

Mtungaji mwepesi na mwenye utendaji bora kwa Muundo wa Maandishi wa WebAssembly (WAT), ulioandikwa kwa Rust.

## Muhtasari

`water` ni kichanganuzi na mtungaji mdogo lakini wenye ufanisi kwa muundo wa maandishi unaosomeka na binadamu wa WebAssembly. Hutoa msingi wa kuchanganua moduli za WAT, maagizo, uingizaji, vitendaji, na vipengele vingine vya WebAssembly.

## Vipengele

- **Uchanganuzi wa haraka** - Uchanganuzi wenye ufanisi wa maagizo na moduli za WAT kwa kutumia `nom`
- **Utegemezi mdogo** - Msingi wa msimbo mwepesi wenye utegemezi muhimu pekee
- **Salama kwa aina** - Hutumia mfumo wa aina wa Rust kwa uwakilishi salama wa AST
- **Inaweza kupanuliwa** - Usanifu wa moduli unaounga mkono watoaji na mabadiliko maalum

## Muundo wa Mradi

- `src/parser/` - Mantiki ya uchanganuzi wa WAT kwa maagizo, vitendaji, uingizaji, na moduli
- `src/emitter/` - Vifaa vya utoaji na ubadilishaji wa msimbo
- `src/ast.rs` - Ufafanuzi wa Mti wa Sintaksia Dhahania (AST)
- `src/leb128.rs` - Usimbaji wa nambari kamili wa urefu unaobadilika wa LEB128
- `src/opcode.rs` - Ufafanuzi wa opcode za WebAssembly

## Kuanza

### Mahitaji ya Awali

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

- **nom** (7.1.1) - Maktaba ya vichanganuzi vya viunganishi

## Leseni

Imetolewa chini ya Leseni ya MIT - tazama faili ya [LICENSE](LICENSE) kwa maelezo zaidi.

## Hali ya Mradi

Huu ni mradi wa majaribio/elimu unaolenga kuelewa uchanganuzi na ukusanyaji wa Muundo wa Maandishi wa WebAssembly.
