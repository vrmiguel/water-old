# water - Ikhompayila Yefomethi Yombhalo ye-WebAssembly

Ikhompayila elula futhi esebenza kahle yefomethi yombhalo ye-WebAssembly (WAT), ebhalwe nge-Rust.

## Ukubuka Kafushane

`water` iyisihlalutyi kanye nekhompayila encane kodwa esebenza kahle yefomethi yombhalo ye-WebAssembly efundeka kubantu. Inikeza isisekelo sokuhlaziya amamojula e-WAT, imiyalelo, imports, functions, kanye nezinye izingxenye ze-WebAssembly.

## Izici

- **Ukuhlaziya okusheshayo** - Ukuhlaziya okusebenzayo kwemiyalelo namamojula e-WAT kusetshenziswa `nom`
- **Ukuncika okumbalwa** - Isizinda sekhodi esilula esinokuncika okubalulekile kuphela
- **Kuphephile ngokohlobo** - Kusebenzisa uhlelo lwezinhlobo lwe-Rust ukuze kube nokumelela okuphephile kwe-AST
- **Kulula ukunweba** - Isakhiwo samamojula esisekela emitters kanye transformations ezenziwe ngokwezifiso

## Isakhiwo Sephrojekthi

- `src/parser/` - Ilogiki yokuhlaziya i-WAT yemiyalelo, functions, imports, namamojula
- `src/emitter/` - Amathuluzi okukhipha ikhodi kanye transformations
- `src/ast.rs` - Izincazelo ze-Abstract Syntax Tree (AST)
- `src/leb128.rs` - Ukubhala ngekhodi kwezinombolo eziphelele ezinobude obuguquguqukayo ze-LEB128
- `src/opcode.rs` - Izincazelo zama-opcode e-WebAssembly

## Ukuqalisa

### Okudingekayo

- Rust 1.56 noma inguqulo entsha

### Ukwakha

```bash
cargo build --release
```

### Ukuqalisa Uhlelo

```bash
cargo run
```

## Okuncike Kukho

- **nom** (7.1.1) - Umtapo wolwazi wama-parser combinators

## Ilayisensi

Inikezwe ngaphansi kweLayisensi ye-MIT - bheka ifayela [LICENSE](LICENSE) ukuze uthole imininingwane.

## Isimo Sephrojekthi

Lena iphrojekthi yokuhlola/yokufunda egxile ekuqondeni ukuhlaziywa nokuhlanganiswa kwefomethi yombhalo ye-WebAssembly.
