# water - Kompilátor textového formátu WebAssembly

Lehký a výkonný kompilátor pro textový formát WebAssembly (WAT), napsaný v Rustu.

## Přehled

`water` je minimální, ale efektivní parser a kompilátor pro člověkem čitelný textový formát WebAssembly. Poskytuje základ pro parsování modulů WAT, instrukcí, importů, funkcí a dalších součástí WebAssembly.

## Funkce

- **Rychlé parsování** - Efektivní parsování instrukcí a modulů WAT pomocí `nom`
- **Minimální závislosti** - Lehká kódová základna pouze s nezbytnými závislostmi
- **Typová bezpečnost** - Využívá typový systém Rustu pro bezpečnou reprezentaci AST
- **Rozšiřitelnost** - Modulární architektura podporující vlastní emitery a transformace

## Struktura projektu

- `src/parser/` - Logika parsování WAT pro instrukce, funkce, importy a moduly
- `src/emitter/` - Nástroje pro generování kódu a transformace
- `src/ast.rs` - Definice abstraktního syntaktického stromu (AST)
- `src/leb128.rs` - Kódování celých čísel proměnné délky LEB128
- `src/opcode.rs` - Definice opcode WebAssembly

## Začínáme

### Předpoklady

- Rust 1.56 nebo novější

### Sestavení

```bash
cargo build --release
```

### Spuštění

```bash
cargo run
```

## Závislosti

- **nom** (7.1.1) - Knihovna parser combinators

## Licence

Licencováno pod licencí MIT - podrobnosti najdete v souboru [LICENSE](LICENSE).

## Stav projektu

Toto je experimentální/vzdělávací projekt zaměřený na porozumění parsování a kompilaci textového formátu WebAssembly.
