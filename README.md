# ternary-compass

**Navigation in ternary state space. Which direction is better? Follow the gradient.**

A compass doesn't tell you where you are — it tells you which direction is north. In ternary state space, the compass tells you which direction improves fitness. If your current state is 0 (neutral) and +1 is better, the compass points toward +1. If -1 is better, it points toward -1. If you're already at the optimum, it points nowhere (0 — stay put).

This crate implements gradient-based navigation through ternary strategy spaces: compute the fitness gradient, follow it to a local optimum, detect when you've arrived, and navigate around barriers when the gradient is flat.

## What's Inside

- **`Compass`** — points toward fitness improvement in ternary state space
- **`gradient(current_state, fitness_fn)`** — compute which ternary direction improves fitness
- **`follow(state, gradient, step_size)`** — take a step in the gradient direction
- **`navigate(state, fitness_fn, max_steps)`** — follow the gradient until convergence
- **`is_local_optimum(state, fitness_fn)`** — no neighbor has higher fitness?
- **`gradient_landscape(states, fitness_fn)`** — compute gradients for the entire state space
- **`barrier_detect(state, fitness_fn)`** — detect flat regions where the gradient is zero but it's not an optimum

## Quick Example

```rust
use ternary_compass::*;

// Fitness function: +1 is optimal
let fitness = |s: i8| -> f64 { s as f64 };

// Start at 0
let mut state = 0;
let path = navigate(state, &fitness, 10);
// 0 → +1 (followed gradient toward higher fitness)

// Check if we've arrived
assert!(is_local_optimum(1, &fitness));
// +1 is a local (and global) optimum

// Barrier detection: fitness is flat between -1 and 0
let flat_fitness = |s: i8| -> f64 { if s == 1 { 1.0 } else { 0.0 } };
let barriers = barrier_detect(0, &flat_fitness);
// Flat region detected: -1 and 0 have the same fitness
```

## The Deeper Truth

**Ternary gradients are the simplest possible optimization.** In continuous space, gradient descent follows a smooth curve toward the optimum. In ternary, there are only three directions: toward -1, stay at 0, or toward +1. The gradient is a single ternary value — the direction of steepest ascent. Navigation is a sequence of ternary decisions: at each step, which of the three states is better?

This simplicity is deceptive. In multi-dimensional ternary spaces (where each agent has N ternary variables), the gradient landscape becomes complex: local optima, saddle points, and flat regions abound. The compass still works — it just might get stuck in local optima. Escaping local optima requires perturbation (random jumps) or momentum (remembering previous directions) — both of which connect to the genetic algorithm (ternary-ga) and simulated annealing approaches.

**Use cases:**
- **Strategy optimization** — find the best ternary strategy by following fitness gradients
- **Navigation** — guide agents through ternary state spaces
- **Landscape analysis** — map the gradient structure of fitness landscapes
- **Education** — the simplest possible optimization algorithm
- **Agent coordination** — agents follow gradients toward consensus

## See Also

- **ternary-fitness** — the landscapes being navigated
- **ternary-ga** — genetic algorithms (gradient-free optimization)
- **ternary-navigator** — higher-level navigation with path planning
- **ternary-gradient** — gradient computation in multi-dimensional ternary spaces
- **ternary-topology** — topological analysis of gradient landscapes

## Install

```bash
cargo add ternary-compass
```

## License

MIT
