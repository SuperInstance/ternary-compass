# Ternary Compass

**Ternary Compass** provides orientation and direction in ternary state space — modeling navigational instruments (compass, gyroscope, sextant) that help agents understand their position, heading, and rate of change within the {-1, 0, +1} landscape.

## Why It Matters

Agents navigating a ternary state space need to know where they are, which direction they're heading, and whether they're moving toward or away from goals. Ternary Compass provides these primitives: bearings (Approach +1, Avoid -1, Neutral 0), heading estimation from state deltas, gyroscopic stabilization against noise, and anomaly detection for disorientation. This is the navigational layer that enables strategic movement rather than random exploration.

## How It Works

### Bearing

The fundamental direction in ternary space:

```
Bearing::Approach  → moving toward +1 (positive growth)
Bearing::Avoid     → moving toward -1 (negative/avoidance)
Bearing::Neutral   → holding at 0 (no change)
```

Bearing from state delta: `bearing = sign(delta)`. Computed in **O(1)**.

### Angular Representation

Bearings map to angles in 2D ternary space:

```
Neutral  → 0 radians
Approach → +π/3 (60°)
Avoid    → -π/3 (-60°)
```

Angular distance between bearings: **O(1)** subtraction.

### Heading Estimation

From a sequence of ternary states, estimate the heading:

```
heading = weighted_average(recent_deltas)
confidence = |heading| / max_possible_heading
```

A strong heading (close to ±1) indicates consistent directional movement. Weak heading (close to 0) indicates random walk or oscillation. Estimation: **O(N)** for N recent states.

### Gyroscope (Stabilization)

The gyroscope filters noise from heading estimates:

```
filtered_heading = α · new_reading + (1 - α) · previous_heading
```

where α is the smoothing factor (0-1). Low α = strong smoothing (slower response). High α = responsive (more noise). This is an exponential moving average filter.

### Sextant (Position Fix)

Given observed bearings to known reference points:

```
position = triangulate(bearings, references)
```

Triangulation: **O(N²)** for N reference points (least-squares optimization).

### Anomaly Detection

Flag disorientation:

```
if heading_variance > threshold:
    agent is disoriented (random walk)
if rate_of_change > max_safe_rate:
    agent is destabilizing
```

Detection: **O(N)** over recent state history.

## Quick Start

```rust
use ternary_compass::{Compass, Bearing, Ternary};

let mut compass = Compass::new();

compass.update(Ternary::Positive);
compass.update(Ternary::Positive);
compass.update(Ternary::Zero);

let heading = compass.estimate_heading();
println!("Bearing: {:?}", Bearing::from_delta(heading));
println!("Confidence: {:.2}", compass.confidence());
```

## API

| Type | Description |
|------|-------------|
| `Compass` | Tracks heading history, estimates direction |
| `Bearing` | Approach (+1), Avoid (-1), Neutral (0) |
| `Gyroscope` | EMA-filtered heading stabilization |
| `Sextant` | Position triangulation from references |
| `Ternary` | Negative (-1), Zero (0), Positive (+1) |

Key methods: `update(state)`, `estimate_heading()`, `confidence()`, `anomaly_detected()`.

## Architecture Notes

Ternary Compass provides the navigational awareness layer for fleet agents in SuperInstance. In γ + η = C, the compass tells agents whether their current trajectory contributes to γ (Approach +1, growth) or η (Avoid -1, avoidance), enabling strategic navigation rather than random search. Integrates with `ternary-cartograph` for map-based navigation and `ternary-anchor` for position holding.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for agent navigation architecture.

## References

1. Borenstein, J. & Feng, L. (1996). "Measurement and Correction of Systematic Odometry Errors in Mobile Robots." *IEEE Transactions on Robotics and Automation*.
2. Thrun, S. et al. (2005). *Probabilistic Robotics*. MIT Press.
3. Kalman, R. E. (1960). "A New Approach to Linear Filtering and Prediction Problems." *Journal of Basic Engineering*, 82(1), 35–45.

## License

MIT
