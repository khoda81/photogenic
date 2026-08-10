# Photogenic

A browser-based genetic algorithm for arranging random colors into visually smooth sequences.

The core search is written in Rust and compiled to WebAssembly. Candidate palettes are permutations of the same colors; fitness rewards adjacent colors that are perceptually similar according to **CIEDE2000** distance. The browser renders the best candidates live while the population evolves.

**Live demo:** https://photogenic-delta.vercel.app

## How it works

1. Generate a set of random RGB colors.
2. Create a population of random permutations of those colors.
3. Score each permutation from the perceptual similarity of adjacent colors.
4. Select parents with fitness-weighted sampling.
5. Produce the next generation with order-preserving crossover and mutation.
6. Keep the current best candidate alive between generations.

Mutations include swapping positions, reversing subsequences, and rotating subsequences. The mutation strategy itself carries probabilities that participate in crossover and mutation.

## Interactive controls

The web UI exposes:

- **Population** — number of candidate orderings evolved at once
- **Number of Colors** — palette size
- **Mutation Rate** — probability of mutating each new child
- **Reset** — generate a new random color set and population

The canvas displays the highest-fitness candidates together with generation and fitness information.

## Tech

- Rust
- WebAssembly / `wasm-bindgen`
- HTML Canvas
- CIEDE2000 perceptual color distance

## Development

Build the WebAssembly package and frontend with the existing npm/webpack setup:

```bash
npm install
npm run build
```

Run the Rust tests with:

```bash
cargo test
```

## Motivation

Sorting colors is a small but useful playground for permutation search: the objective is intuitive enough to inspect visually, while the search space grows factorially with the number of colors. That makes it a neat test bed for crossover, mutation, and selection strategies with immediate visual feedback.
