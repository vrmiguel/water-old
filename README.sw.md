# water - Mkutano wa Format ya Maandishi ya WebAssembly

> **[Soma katika Kiswahili](README.sw.md)**

Mkutani mringi na wenye utendaji mzuri wa Format ya Maandishi ya WebAssembly (WAT), iliyoandikwa kwa Rust.

## Muhtasari

`water` ni msambazaji wa lugha na mkutani mdogo lakini wenye utendaji mzuri kwa muundo wa maandishi wanayosomeka wa binadamu wa WebAssembly. Inatoa msingi wa kuchanganua moduli za WAT, maagizo, uingizaji, kazi na sehemu nyingine za WebAssembly.

## Sifa

- **Kuchanganua Haraka** - Kuchanganua kwa ufanisi wa maagizo na moduli za WAT kwa kutumia `nom`
- **Utegemezi Mdogo** - Msingi wa msimbo mwenye uzani mdogo na utegemezi wa muhimu tu
- **Salama ya Aina** - Inatumia mfumo wa aina wa Rust kwa kuwakilisha kwa usalama wa AST
- **Inaenezwa** - Usanifu wa moduli unaouunga mkono wanatoa nchi nyumbani na mabadiliko ya kaida

## Muundo wa Mradi

- `src/parser/` - Mantiki ya kuchanganua WAT kwa maagizo, kazi, uingizaji na moduli
- `src/emitter/` - Zana za kutoa na kubadilisha nchi
- `src/ast.rs` - Ufafanuzi wa Mti wa Syntax Abstract (AST)
- `src/leb128.rs` - Kigezo cha LEB128 cha urefu unaobadilika
- `src/opcode.rs` - Ufafanuzi wa opcodes wa WebAssembly

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

## Utegemezi

- **nom** (7.1.1) - Kituo cha wachanganuzaji wa kaida

## Leseni

Iliyopendekezwa chini ya Leseni ya MIT - tazama faili ya [LICENSE](LICENSE) kwa maelezo zaidi.

## Hali ya Mradi

Hii ni mradi wa majaribio/elimu unaotumaini kuelewa uchambuzi wa Format ya Maandishi ya WebAssembly na mkutano.
