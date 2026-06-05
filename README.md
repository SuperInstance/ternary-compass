# ternary-compass

**Orientation and direction in ternary state space**

[![ternary](https://img.shields.io/badge/ecosystem-ternary-blue)](https://github.com/orgs/SuperInstance/repositories?q=ternary)
[![tests](https://img.shields.io/badge/tests-27-green)]()

## Overview

Orientation and direction in ternary state space.

Provides navigational metaphors for understanding position and movement
within ternary {-1, 0, +1} state spaces: compass bearings, gyroscopic
stabilization, angular measurement, heading estimation, and anomaly detection.

## Architecture

- **`Compass`** — core data structure
- **`Gyroscope`** — core data structure
- **`Sextant`** — core data structure
- **`HeadingEstimator`** — core data structure
- **`CompassRose`** — core data structure
- **`MagneticAnomaly`** — core data structure
- **`AnomalyDetector`** — core data structure
- **`Ternary`** — state enumeration
- **`Bearing`** — state enumeration
- **`TernaryDirection`** — state enumeration
- **`AnomalyKind`** — state enumeration

### Key Functions

- `value()`
- `from_value()`
- `from_delta()`
- `to_angle()`
- `new()`
- `update()`
- `bearing()`
- `confidence()`
- `reset()`
- `new()`
- ... and 20 more

## Why Ternary?

The balanced ternary system {-1, 0, +1} (also known as Z₃) is the mathematically optimal discrete encoding:
- **More expressive than binary**: three states capture positive, neutral, and negative
- **Natural for decisions**: accept/reject/abstain, buy/hold/sell, agree/disagree/neutral
- **Self-balancing**: the 0 state acts as a universal screen, preventing pathological lock-in
- **Z₃ cyclic dynamics**: rock-paper-scissors is the only natural coordination mechanism

## Stats

| Metric | Value |
|--------|-------|
| Lines of Rust | 778 |
| Test count | 27 |
| Public types | 11 |
| Public functions | 30 |

## Ecosystem

This crate is part of the **[SuperInstance Ternary Fleet](https://github.com/orgs/SuperInstance/repositories?q=ternary)**:

- **[ternary-core](https://github.com/SuperInstance/ternary-core)** — shared traits and Z₃ arithmetic
- **[ternary-grid](https://github.com/SuperInstance/ternary-grid)** — spatial grid with {-1, 0, +1} cells
- **[ternary-graph](https://github.com/SuperInstance/ternary-graph)** — ternary-weighted graph algorithms
- **[ternary-automata](https://github.com/SuperInstance/ternary-automata)** — three-state cellular automata
- **[ternary-compiler](https://github.com/SuperInstance/ternary-compiler)** — expression compiler and optimizer

200+ crates. 4,300+ tests. One pattern.

## Research Context

The ternary approach connects to several active research areas:
- **Ternary Neural Networks** (TNNs): weights constrained to {-1, 0, +1} for efficient inference
- **Huawei's ternary chip**: 7nm ternary silicon with 60% less power consumption
- **Active inference**: free energy minimization naturally maps to ternary action selection
- **Cyclic dominance**: RPS dynamics maintain biodiversity in spatial ecology
- **Z₃ group theory**: the only algebraic group on three elements is cyclic addition mod 3

## Usage

```toml
[dependencies]
ternary-compass = "0.1.0"
```

```rust
use ternary_compass;
```

## License

MIT
