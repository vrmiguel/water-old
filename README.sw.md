# water - Mkusanyaji wa Muundo wa Maandishi wa WebAssembly

Mkusanyaji mwepesi na wenye utendaji wa hali ya juu kwa Muundo wa Maandishi wa WebAssembly (WAT), ulioandikwa katika Rust.

## Muhtasari

`water` ni mchanganuzi na mkusanyaji mdogo lakini wenye ufanisi kwa muundo wa maandishi unaosomeka na binadamu wa WebAssembly. Hutoa msingi wa kuchanganua moduli za WAT, maagizo, viingilio, vitendaji, na vipengele vingine vya WebAssembly.

## Sifa

- **Uchanganuzi wa haraka** - Uchanganuzi wenye ufanisi wa maagizo na moduli za WAT kwa kutumia `nom`
- **Utegemezi mdogo** - Msingi mwepesi wa msimbo wenye utegemezi muhimu pekee
- **Salama kwa aina** - Hutumia mfumo wa aina za Rust kwa uwakilishi salama wa AST
- **Inayoweza kupanuliwa** - Usanifu wa moduli unaounga mkono vitoaji na mabadiliko maalum

## Muundo wa Mradi

- `src/parser/` - Mantiki ya uchanganuzi wa WAT kwa maagizo, vitendaji, viingilio na moduli
- `src/emitter/` - Zana za utoaji na ubadilishaji wa msimbo
- `src/ast.rs` - Ufafanuzi wa Mti wa Sintaksia Dhahania (AST)
- `src/leb128.rs` - Usimbaji wa nambari kamili wa urefu unaobadilika wa LEB128
- `src/opcode.rs` - Ufafanuzi wa opkodi za WebAssembly

## Kuanza

### Mahitaji ya Awali

- Rust 1.56 au matoleo mapya zaidi

### Kujenga

```bash
cargo build --release
```

### Kuendesha

```bash
cargo run
```

## Utegemezi

- **nom** (7.1.1) - Maktaba ya viunganishi vya wachanganuzi

## Leseni

Imetolewa leseni chini ya Leseni ya MIT - tazama faili la [LICENSE](LICENSE) kwa maelezo zaidi.

## Hali ya Mradi

Huu ni mradi wa kimajaribio/wa kielimu unaolenga kuelewa uchanganuzi na ukusanyaji wa Muundo wa Maandishi wa WebAssembly.
