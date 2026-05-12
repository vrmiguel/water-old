# water - Mkusanyaji wa Muundo wa Maandishi WebAssembly

> **[Kusoma kwa Kiingereza](README.md)** | **[Kusoma kwa Kifaransa](README.fr.md)**

Mkusanyaji mzuri na wenye utendaji wa juu wa Muundo wa Maandishi WebAssembly (WAT), uliofungwa katika Rust.

## Muhtasari

`water` ni msambaji na mkusanyaji mdogo lakini wenye ufanisi wa muundo wa maandishi wa WebAssembly unaoelezwa kwa lugha ya binadamu. Inatoa msingi wa kusambaza moduli za WAT, maagizo, kuagiza, kazi, na sehemu nyingine za WebAssembly.

## Sifa

- **Kusambaza kwa kasi** - Kusambaza WAT maagizo na moduli kwa ufanisi kwa kutumia `nom`
- **Tegemezi ndogo** - Msingi wa nambari mzuri na tegemezi tu muhimu
- **Salama kwa aina** - Inatumia mfumo wa aina wa Rust kwa uwakilishi salama wa AST
- **Upanuaji** - Usanidi wa moduli unasaidiana na watumaji wa kaida na mabadiliko ya kaida

## Muundo wa Mradi

- `src/parser/` - Mantiki ya kusambaza WAT kwa maagizo, kazi, kuagiza, na moduli
- `src/emitter/` - Zana za kutoa nambari na mabadiliko ya nambari
- `src/ast.rs` - Ufafanuzi wa Mti wa Sintaksi Dhahania (AST)
- `src/leb128.rs` - Usimbaji wa nambari wa urefu wa kigezo LEB128
- `src/opcode.rs` - Ufafanuzi wa opcodes wa WebAssembly

## Kuanza

### Mahitaji ya Awali

- Rust 1.56 au nyaraka

### Kujenga

```bash
cargo build --release
```

### Kuendesha

```bash
cargo run
```

## Tegemezi

- **nom** (7.1.1) - Kitabu cha kombineta cha msambaji

## Leseni

Iliyoorodhesha chini ya Leseni ya MIT - tazama faili la [LICENSE](LICENSE) kwa maelezo.

## Hali ya Mradi

Hii ni mradi wa majaribio/elimu unaozingatia kuelewa kusambaza na kukusanya muundo wa maandishi wa WebAssembly.
