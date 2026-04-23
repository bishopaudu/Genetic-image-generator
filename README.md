# 🧬 Genetic Image Generator

> A genetic algorithm that evolves populations of colored shapes to recreate target images — built from first principles in Rust.

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)
![Status](https://img.shields.io/badge/Status-Active-brightgreen?style=flat-square)

---

## What Is This?

**Genetic Image Generator** is a Rust implementation of a genetic algorithm that evolves a population of images — each made of colored, semi-transparent triangles and circles — until they visually resemble a target photograph or image.

No neural networks. No gradient descent. No hand-crafted rules.

Just **selection, crossover, and mutation** — the same forces that shape life on Earth — applied to pixels.

The result is a program that starts with 100 random collections of shapes and, over thousands of generations, gradually sculpts them into a surprisingly faithful geometric portrait of the target.

```
Generation 0        Generation 500       Generation 2000      Generation 5000
┌──────────┐        ┌──────────┐         ┌──────────┐         ┌──────────┐
│ random   │        │ rough    │         │ clear    │         │ detailed │
│ colored  │  ───►  │ color    │  ───►   │ shapes   │  ───►   │ geometric│
│ blobs    │        │ regions  │         │ visible  │         │ portrait │
└──────────┘        └──────────┘         └──────────┘         └──────────┘
```

---

## Features

- **Pure genetic algorithm** — selection, crossover, mutation, elitism
- **Two shape types** — circles and triangles with full RGBA color support
- **Tournament selection** — configurable pressure balancing quality and diversity
- **Uniform crossover** — each shape independently inherited from either parent
- **Two-tier mutation** — small nudges (90%) and full shape replacement (10%)
- **Elitism** — best genome always preserved unchanged across generations
- **Live visual display** — side-by-side window showing target vs current best
- **Real-time terminal progress bar** — fitness, speed, and ETA
- **Automatic image saving** — snapshots every N generations
- **Fully configurable** — shapes, population size, mutation rate, and more
- **Release mode optimized** — runs 10–20× faster with `cargo run --release`

---

## How It Works

### The Core Idea

Every image in the population is represented as a **genome** — an ordered list of shape descriptions (position, size, color, transparency). The algorithm never manipulates pixels directly. It only ever creates, scores, and modifies these shape lists.

```
Genome = [
  Circle  { x: 142, y: 83,  radius: 67, color: (201, 45, 130, 88)  },
  Triangle{ (12,190)(88,34)(177,120),    color: (55, 200,  17, 142) },
  Circle  { x: 9,   y: 171, radius: 23, color: (88,  12, 200,  61) },
  ... 47 more shapes ...
]
```

### The Evolution Loop

Each generation runs these steps in order:

```
1. SCORE      Render each genome → compare pixels to target → assign error
2. ELITE      Copy the best genome unchanged into the next generation
3. SELECT     Run tournaments — better genomes win more often
4. CROSSOVER  For each child: pick each shape from either parent (50/50)
5. MUTATE     Randomly nudge shape properties (position, size, color)
6. REPEAT     The children become the new population
```

### Why It Works

Semi-transparent shapes **blend together like watercolors**. Large shapes establish rough color regions. Smaller shapes add detail. Over thousands of generations, shapes migrate to the right positions, colors drift toward the target values, and a recognizable image emerges — purely through fitness pressure.

---

## Quickstart

### Prerequisites

- Rust 1.70 or higher ([install here](https://rustup.rs))
- A target image (PNG or JPEG, ideally 200×200 or smaller)

### Installation

```bash
git clone https://github.com/bishopaudu/Genetic-image-generator.git
cd genetic-image-generator
```

### Run With the Built-in Test Target

```bash
cargo run --release
```

This evolves toward a simple red circle + blue triangle. A window will open showing the target (left) and the evolving image (right).

### Run With Your Own Image

1. Place your image in the project root folder:
   ```
   genetic-image-generator/
   ├── Cargo.toml
   ├── your_photo.png   ← here
   └── src/
   ```

2. Edit `src/main.rs` in the `main` function:
   ```rust
   // Replace this line:
   let target = create_test_target(200, 200);

   // With this:
   let target = image::open("your_photo.png")
       .expect("Could not open image")
       .to_rgba8();
   ```

3. Run:
   ```bash
   cargo run --release
   ```

---

## Configuration

All algorithm parameters are controlled through the `EvolutionConfig` struct:

```rust
let config = EvolutionConfig {
    width:           200,     // canvas width in pixels
    height:          200,     // canvas height in pixels
    num_shapes:      50,      // shapes per genome
    population_size: 100,     // number of genomes in the population
    tournament_size: 5,       // candidates per selection tournament
    mutation_rate:   0.02,    // probability each shape is mutated (per generation)
    num_generations: 5000,    // how many generations to run
    save_every:      500,     // save a PNG snapshot every N generations
    display_every:   10,      // update the live window every N generations
    output_dir:      "output".to_string(),
};
```

### Recommended Settings by Image Type

| Image Type | `num_shapes` | `population_size` | `num_generations` | Notes |
|---|---|---|---|---|
| Simple (flag, logo) | 30 | 100 | 5,000 | Converges quickly |
| Portrait | 100 | 100 | 20,000 | Good detail level |
| Complex scene | 200 | 50 | 50,000 | Reduce population for speed |
| Experimentation | 50 | 100 | 2,000 | Fast iteration |

---

## Output

While running, the program produces:

```
output/
├── gen_00500.png    ← snapshot at generation 500
├── gen_01000.png    ← snapshot at generation 1000
├── gen_01500.png    ← snapshot at generation 1500
└── ...

target.png           ← copy of the target image
final_result.png     ← best genome at end of run
```

### Terminal Display

```
╔════════════════════════════════════╗
║   Genetic Image Evolution  v1.0    ║
╚════════════════════════════════════╝

Target: 200×200 pixels
Creating random population of 100 genomes...
Generation 0 fitness: 79.34%

Evolution running — press ESC to stop early

 Gen   470/5000 │████████████░░░░░░░░░░░░░░░░░░│ 82.14% │ Best: 82.14% │ 2.4 gen/s │ ETA: 1m53s ↑
```

### Live Window

A window opens showing:
- **Left half** — the target image (static)
- **Right half** — the current best genome (updates every 10 generations)

Press **ESC** or close the window to stop evolution early.

---

## What to Expect

Fitness measures how closely the evolved image matches the target:

| Fitness | What You See |
|---|---|
| 75–80% | Rough color blobs, dominant colors visible |
| 80–85% | Major regions taking shape, clear color zones |
| 85–90% | Recognizable resemblance, geometric structure visible |
| 90–93% | Good approximation, looks like deliberate geometric art |
| 93–96% | Impressive detail, fine features beginning to appear |
| 96%+   | Striking result — requires many shapes and long runtime |

### Typical Progress Curve

Improvement is rapid early, then gradually slows:

```
  96% ┤                                              ╭────
  94% ┤                                        ╭─────╯
  92% ┤                                  ╭─────╯
  90% ┤                           ╭──────╯
  88% ┤                    ╭──────╯
  86% ┤             ╭──────╯
  84% ┤      ╭──────╯
  82% ┤╭─────╯
  80% ┼┴──────────────────────────────────────────────
      0    1000    2000    3000    4000    5000   generations
```

This is normal — called **diminishing returns**. Early improvements are easy to find. Later ones require finer tuning.

---

## Algorithm Deep Dive

### Genome Representation

Each shape is stored as a Rust enum with two variants:

```rust
enum Shape {
    Circle   { x, y, radius, color: Color },
    Triangle { x1, y1, x2, y2, x3, y3, color: Color },
}

struct Color { r: u8, g: u8, b: u8, a: u8 }  // RGBA, 0–255 each
```

Alpha (transparency) is kept between 30–200 — visible enough to contribute, transparent enough to blend with other shapes.

### Fitness Function

Uses **Sum of Squared Differences (SSD)** across all pixels:

```
error = Σ (ΔR² + ΔG² + ΔB²)  for every pixel
```

Squaring the differences penalizes large errors more than small ones. Only RGB channels are compared — alpha is excluded as it's a rendering tool, not a target property.

### Selection: Tournament

A group of `tournament_size` random genomes compete. The lowest-error genome wins and becomes a parent. Repeated `population_size` times to fill the parent pool.

This balances selection pressure: better genomes win more often but weaker ones occasionally survive — maintaining genetic diversity and avoiding premature convergence.

### Crossover: Uniform

For each shape position, a coin is flipped. The child inherits that shape from either parent A or parent B with equal probability:

```
Parent A: [🔴, 🔵, 🟢, 🟡, 🟣]
Parent B: [🔷, 🔶, 🔸, 🔹, 💠]
Coin:     [ H,  T,  H,  T,  H ]
Child:    [🔴, 🔶, 🟢, 🔹, 🟣]
```

Each shape is an independent gene — uniform crossover respects this by treating each one separately.

### Mutation: Two-Tier

With probability `mutation_rate` (default 2%), each shape is mutated:

- **90% chance** — tweak existing shape: nudge position, size, and/or color by small random amounts
- **10% chance** — replace the entire shape with a new random one (escapes local optima)

Color and position nudges are clamped to valid ranges after each mutation.

### Elitism

The best genome from each generation is always copied unchanged into the next. This guarantees fitness can **never decrease** — the best solution found is always preserved.

---

## Project Structure

```
genetic-image-generator/
├── Cargo.toml          # dependencies and project metadata
├── Cargo.lock          # locked dependency versions
├── README.md           # this file
├── src/
│   └── main.rs         # entire implementation (~500 lines)
├── output/             # generated during a run — progress snapshots
├── target.png          # generated during a run — copy of target
└── final_result.png    # generated during a run — best result
```

### Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `rand` | 0.8 | Random number generation |
| `image` | 0.24 | Image loading, saving, pixel manipulation |
| `imageproc` | 0.23 | Drawing filled shapes onto images |
| `minifb` | 0.25 | Live display window |

---

## Tips for Best Results

**Choose the right image:**
- Bold, high-contrast images with simple subjects work best
- Portraits, logos, and iconic artwork converge beautifully
- Fine textures (grass, hair, fabric) are difficult to approximate
- Resize your image to 200×200 or smaller before using it

**Tune the parameters:**
- More shapes = more detail but slower per generation
- Larger population = more diversity but slower scoring
- Higher mutation rate = more exploration but risk of destroying good solutions
- Run overnight with 50,000+ generations for impressive results

**Use release mode:**
```bash
cargo run --release   # 10–20× faster than debug mode
```

---

## Inspiration and Background

This project is inspired by the famous **genetic programming art** experiments that became popular in the early 2000s, most notably Roger Alsing's 2008 blog post where he evolved the Mona Lisa using only 50 semi-transparent polygons.

The core insight — that semi-transparent overlapping shapes can approximate complex images through evolutionary pressure alone — remains one of the most elegant demonstrations of what genetic algorithms can achieve.

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

## Author

Built step by step, from first principles, as a deep learning exercise in both **genetic algorithms** and **Rust programming**.

> *"Evolution doesn't design. It discovers."*
