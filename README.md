# Tile Floor Pattern Generator

A program that finds all possible patterns for placing colored tiles on a 2×5 floor grid.

## Problem

Place red, blue, and yellow tiles on a 2×5 floor grid with the following conditions:
1. Use 4 red tiles, 4 blue tiles, and 2 yellow tiles
2. Tiles of the same color cannot be adjacent to each other

See the implementation details in [`src/lib.rs`](src/lib.rs).

## Demo

Visit the [GitHub Pages demo](https://bem130.github.io/tmp20250516/) to see all valid patterns.
- Patterns can be reordered by drag & drop
- Touch-enabled for mobile devices

## Technical Stack

- **Frontend**: HTML, CSS, JavaScript
- **Backend**: Rust + WebAssembly
- **Build**: wasm-pack
- **Deployment**: GitHub Actions + GitHub Pages

## Development

```bash
# Install Rust and wasm-pack
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build
wasm-pack build --target web --out-dir dist --features wasm
```
