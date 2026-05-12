# water - Tafsiri ya Programu ya WebAssembly Text Format

> **[Soma kwa Kiingereza](README.md)** | **[Soma kwa Kifaranga](README.fr.md)**

Kimpileta nyepesi na wenye utendaji mzuri kwa WebAssembly Text Format (WAT), iliyoandikwa kwa Rust.

## Muhtasari

`water` ni tafsiri na kimpileta yenye ustadi mdogo lakini wenye utendaji mzuri kwa muundo wa maandishi swa WebAssembly unaoendelea kusomwa. Inatoa msingi kwa ajili ya kutatua WAT modules, maagizo, nyingine, na sehemu zingine za WebAssembly.

## Sifa

- **Tafsiri haraka** - Kufafanua kwa ufanisi WAT instruction na module kwa kutumia `nom`
- **Utegemezi mdogo** - Kod yenye uzani mdogo wenye tu utegemezi wa lazima
- **Salama kwa aina** - Inatumia mfumo wa aina wa Rust kwa ajili ya uwakilishi wa AST salama
- **Panulizi-inayoweza** - Usanifu wa moduli unaotumika inayotumia vitendaji vya utoaji na urekebaji vya kawaida

## Muundo wa Mradi

- `src/parser/` - Mantiki ya kutatua WAT kwa maagizo, kazi, nyingine, na modules
- `src/emitter/` - Ajili za kutoa kod na urekebaji wa kawaida
- `src/ast.rs` - Ufafanuzi wa Abstract Syntax Tree
- `src/leb128.rs` - Usimbaji wa nambari ndefu ya kigezo cha LEB128
- `src/opcode.rs` - Ufafanuzi wa opcode wa WebAssembly

## Kuanza

### Mahitaji Kabla

- Rust 1.56 au zaidi

### Kujenga

```bash
cargo build --release
```

### Kuendesha

```bash
cargo run
```

## Utegemezi

- **nom** (7.1.1) - Maktaba ya viambatanisho vya kutatua

## Leseni

Leseni iliyo chini ya MIT License - tazama faili la [LICENSE](LICENSE) kwa maelezo.

## Hali ya Mradi

Hii ni mradi wa ujifunzaji/elimu unaozingatia kuelewa kutatua muundo wa WAT na ukompilesheni.
