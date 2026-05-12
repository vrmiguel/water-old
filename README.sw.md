# water - Kichakataji cha Muundo wa Maandishi ya WebAssembly

> **[Lire en français](README.fr.md) | [Read in English](README.md)**

Kichakataji ringan na kizalishaji kwa Muundo wa Maandishi ya WebAssembly (WAT), kilichoandikwa katika Rust.

## Muhtasari

`water` ni kichakataji na kichakataji kidogo lakini kizalishaji kwa muundo wa maandishi wa WebAssembly wenye ufahamu. Inatoa msingi wa kuchakata moduli za WAT, maagizo, kunauza, vitendaji, na sehemu nyingine za WebAssembly.

## Sifa

- **Kuchakata haraka** - Kuchakata kwa ufanisi kwa maagizo ya WAT na moduli kwa kutumia `nom`
- **Utegemezi mdogo** - Koodi hafif na utegemezi tu wa muhimu
- **Salama kwa aina** - Inatumia mfumo wa aina wa Rust kwa uwakilishi salama wa AST
- **Inaweza kupanuliwa** - Usanidi wa msimu unaounga mkono wazazi wa kawaida na mabadiliko

## Muundo wa Mradi

- `src/parser/` - Mantiki ya kuchakata WAT kwa maagizo, vitendaji, kunauza, na moduli
- `src/emitter/` - Mikakati ya kusambaza nambari na mabadiliko
- `src/ast.rs` - Ufafanuzi wa Muundo wa Abstracto wa Syntax
- `src/leb128.rs` - Kodishaji la LEB128 ya nambari zenye urefu tofauti
- `src/opcode.rs` - Ufafanuzi wa nambari za amri za WebAssembly

## Kuanza

### Mahitaji Yaliyotakarika

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

- **nom** (7.1.1) - Maktaba ya kichakataji cha umoja

## Leseni

Leseni chini ya Leseni ya MIT - angalia faili la [LICENSE](LICENSE) kwa maelezo.

## Hali ya Mradi

Hii ni mradi wa majaribio/elimu unaozingatia kuelewa kuchakata na kukamatia muundo wa maandishi ya WebAssembly.
