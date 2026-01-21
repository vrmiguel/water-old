# water (Tekstoformata WebAssembly-kompilaŭilo)

`water` celas esti malgranda kaj nepre rapida tekstoformata WebAssembly-kompilaŭilo.

## Superrigardo

`water` estas kompilaŭilo dizajnita por transformi WebAssembly-tekstoformon (WAT) en binarajn WebAssembly-kodojn. Ĝi fokusiĝas sur rapidaj kompilaĵoj kun minimuma grandeco, kio ĝin taŭgas por integritaj sistemoj kaj nepre rapidaj medioj.

## Trajtoj

- **Malpezo**: Minimumaj dependoj kaj malgranda grandeco de binaroj
- **Rapida kompilaĵo**: Optimumigita por rapidaj kompilaĵtempoj
- **WAT-subteno**: Vasta subteno por WebAssembly-tekstoformoj
- **Plurplataforma**: Fluas en diversaj operaciumoj

## Komenci

### Instalado

Por komenci kun `water`, ĉonu ĉi tiun arkivon kaj konstruu ĝin:

```bash
cargo build --release
```

### Uzo

Kompilu WAT-dosieron al WebAssembly-binaroj:

```bash
water input.wat -o output.wasm
```

## Kontribuo

Kontribuoj estas bonvenaj! Ĉu vi trovos cimeton, havas funkcion-peton, aŭ deziras plibonigi la dokumentaron, bonvolu malfermi problemon aŭ sendi puŝodemando.

## Permeso

Vidu la PERMESO-dosieron por pli da informo.
