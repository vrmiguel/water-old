# water (Tekstoformata WebAssembly-kompilaŭilo)

`water` celas esti malgranda kaj nepre rapida tekstoformata WebAssembly-kompilaŭilo, disegnita por alporti efikajn WAT-al-WASM-kompilaĵojn al programistoj, kiuj ĉefas sur la rendimento kaj minimuma uzado de rimedoj.

## Superrigardo

`water` estas kompilaŭilo dizajnita por transformi WebAssembly-tekstoformon (WAT) en binarajn WebAssembly-kodojn. Ĝi fokusiĝas sur rapidaj kompilaĵoj kun minimuma grandeco, kio ĝin taŭgas por integritaj sistemoj kaj nepre rapidaj medioj. La projekto kombinas la sekurecan garantion de Rust kun optimumigitaj kompilaĵstrategijoj por liveri fidinda kaj efikajn ĵujojn.

## Motivacio

WebAssembly estas transformrevola teknologio por portebla, altfunkciperfekta kodekzekuto ĉe ĉiaj platformoj. Tamen, multaj WAT-kompilaŭiloj venas kun signifaj ĉarĝoj aŭ dependecoj. `water` traktas ĉi tiun mankon per dispono de malgranda, rapida alternativo, kiu konservas plenan kongruecon kun la WebAssembly-specifo dum minimumigo de rimeda konsumo.

## Trajtoj

- **Malpezo**: Minimumaj dependoj kaj malgranda grandeco de binaroj por facila distribuo kaj enigo
- **Rapida kompilaĵo**: Optimumigita por rapidaj kompilaĵtempoj sen malhelpado de ĝusteco
- **WAT-subteno**: Vasta subteno por WebAssembly-tekstoformat-specifoj
- **Plurplataforma**: Fluas en diversaj operaciumoj kaj arkitekturoj
- **Fidinda**: Konstruita en Rust kun forta tipfsekureco kaj memora garantio
- **Eniga**: Povas esti integrita en pli grandaj projektoj kiel biblioteko

## Projekta strukturo

La projekto estas organita por konservi klareco kaj modulareco:
- **src/**: Ĉefkompilaĵa realigo
- **tests/**: Ampleksa teststupo por validado
- **examples/**: Specimenaj WAT-dosieroj montrante kompilaĵan uzadon

## Komenci

### Antaŭkondiĉoj

- Rust 1.70 aŭ pli nova
- Cargo-ĝujo-mastrumilo

### Instalado

Por komenci kun `water`, ĉonu ĉi tiun arkivon kaj konstruu ĝin:

```bash
git clone <arkiv-URL>
cd water
cargo build --release
```

La kompilita binaroj estos havebla ĉe `target/release/water`.

### Uzo

#### Komanda linio

Kompilu WAT-dosieron al WebAssembly-binaroj:

```bash
water input.wat -o output.wasm
```

#### Kiel biblioteko

Vi ankaŭ povas uzi `water` kiel dependecon en viaj Rust-projektoj:

```toml
[dependencies]
water = { path = "./water" }
```

## Ekzemploj

### Baza WAT-modulo

```wat
(module
  (func $add (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add)
  (export "add" (func $add)))
```

Kompilu ĉi tion per:
```bash
water example.wat -o example.wasm
```

## Testado

Ruladu la teststupon por certiği, ke ĉio funkcias ĝuste:

```bash
cargo test
```

Por linta kaj formato-kontrolo:
```bash
cargo clippy -- -D warnings
cargo fmt -- --check
```

## Rendimento

`water` estas optimumigita por ambaŭ kompilaĵ-rapideco kaj binara grandeco:
- Minimuma memora humo dum kompilaĵo
- Rekta WAT-al-WASM-traduko sen senbezona intermo-reprezentado
- Taŭga por rimedo-limigitaj medioj

## Kontribuo

Kontribuoj estas bonvenaj! Ĉu vi trovos cimeton, havas funkcion-peton, aŭ deziras plibonigi la dokumentaron, bonvolu malfermi problemon aŭ sendi puŝodemando. Ni aprezas ĉiajn formojn de kontribuo, de kodaj plibonigoj ĝis dokumentara plibonigoj.

### Evoluaj gvidoj

1. Duplicu la arkivon
2. Kreu branĉon de trajtoj por viaj ŝanĝoj
3. Certigu, ke ĉiuj testoj pasas kaj kodo estas ĝuste formato
4. Sendi puŝodemando kun klara priskribo de viaj ŝanĝoj

## Permeso

Vidu la PERMESO-dosieron por pli da informo.
