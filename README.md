# Game of Life for WASM-4

A game written in Rust for the [WASM-4] fantasy console.

<p align="center">
    <img src="./doc/animation.gif" alt="animation">
</p>

**:rocket: [Play it here!](https://srynetix.github.io/wasm4-gol/)**  
**:book: [Read the engine documentation here!](https://srynetix.github.io/wasm4-sx/wasm4_sx/)**

## Features

- No memory allocation (and no-std)
- Configurable grid size using const values (just change the `CELL_SIZE` value in the [crates/wasm4-gol/src/logic.rs](./crates/wasm4-gol/src/logic.rs) file)
- Uses a simple ad-hoc WASM-4 wrapper called [wasm4-sx]
- Uses a custom made "tracker" to generate the soundtrack: [wasm4-tracker]

## Dependencies

- The [WASM-4] CLI (`w4`)
- A stable [Rust] compiler with the `wasm32-unknown-unknown` target
- The [Just] command runner
- [wasm4-tracker] to build the music track from the YAML file
    - `cargo install --git https://github.com/Srynetix/wasm4-tracker`

For release builds:
- The `wasm-opt` tool from the [binaryen] project
- The `wasm-strip` tool from the [wabt] project

For WASM analysis (`analyze` tasks):
- Optionally, for WASM analysis, the [twiggy] tool

## Building and running

You can type `just` to see the different build and export tasks.  
Here are some examples:

```sh
# Build the cartridge in release mode + strip and run WASM-4 on web
just run-release-web

# Build the cartridge in release mode + strip and run WASM-4 on native mode
just run-release-native

# Build the cartridge in release mode + strip and export to HTML file
just export-release-web

# Build the cartridge in release mode + strip and export to Windows EXE file
just export-release-exe
```

[WASM-4]: https://wasm4.org
[Rust]: https://www.rust-lang.org/
[binaryen]: https://github.com/WebAssembly/binaryen
[wabt]: https://github.com/WebAssembly/wabt
[Just]: https://github.com/casey/just
[twiggy]: https://github.com/rustwasm/twiggy

[wasm4-sx]: https://github.com/Srynetix/wasm4-sx
[wasm4-tracker]: https://github.com/Srynetix/wasm4-tracker
