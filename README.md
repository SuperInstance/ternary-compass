# ternary-compass: Orientation and direction in ternary state space

Navigational instruments for understanding position and movement within {-1, 0, +1} state spaces.

## Why This Exists

When ternary agents transition between states, you need to know which direction they're heading and how fast. Raw state vectors don't tell you that. This crate provides compass-like instruments — bearings, gyroscopes, sextants — that give you orientation, angular measurement, and anomaly detection in ternary space. Without it, you'd just be staring at sequences of -1s, 0s, and +1s with no sense of trajectory.

## Core Concepts

- **Ternary**: A value in {-1, 0, +1}. Negative, Zero, or Positive.
- **Bearing**: The fundamental direction: Approach (toward +1), Avoid (toward -1), or Neutral (holding at 0).
- **Compass**: Tracks current bearing using majority-vote smoothing over a history window.
- **Gyroscope**: Stabilizes orientation by applying exponential damping to filter transient noise.
- **Sextant**: Measures angles between ternary states, mapped to a circle at ±60° intervals.
- **HeadingEstimator**: Predicts future direction using weighted linear regression on state history.
- **CompassRose**: 8-directional ternary compass for 2D state pairs (analogous to cardinal/ordinal directions).
- **MagneticAnomaly**: Distortions detected in state space — reversals, oscillations, stagnation, unexpected transitions.

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-compass = "0.1"
```

```rust
use ternary_compass::*;

// Create a compass with smoothing window of 5
let mut compass = Compass::new(5);

// Feed state transitions
compass.update(Ternary::Zero, Ternary::Positive);   // Approach
compass.update(Ternary::Positive, Ternary::Positive); // Neutral (no change)
compass.update(Ternary::Positive, Ternary::Negative); // Avoid

println!("Bearing: {:?}, Confidence: {:.2}", compass.bearing(), compass.confidence());

// Use a gyroscope for stabilized orientation
let mut gyro = Gyroscope::new(0.7);
let angle = gyro.stabilize(Bearing::Approach);
println!("Stabilized orientation: {:.3} rad", angle);

// Measure angles between states
let angle = Sextant::angle_between(Ternary::Negative, Ternary::Positive);
println!("Angle: {:.3} rad", angle);
```

## API Overview

| Type | Description |
|------|-------------|
| `Compass` | Tracks bearing with history-based smoothing |
| `Gyroscope` | Stabilizes orientation with configurable damping |
| `Sextant` | Measures angles and traversal in state space |
| `HeadingEstimator` | Predicts future direction from state trends |
| `CompassRose` | 8-directional compass for 2D ternary pairs |
| `TernaryDirection` | The 8 compass directions |
| `AnomalyDetector` | Detects state space distortions and anomalies |
| `Bearing` | Three-way direction: Approach, Avoid, Neutral |

## How It Works

States are mapped to angles on a circle: -1 → -π/3, 0 → 0, +1 → +π/3. This gives each ternary value a 60° separation, making angular measurement natural.

The **compass** maintains a sliding window of recent bearings and uses majority vote to determine the current direction. Confidence is the fraction of history agreeing with the dominant bearing.

The **gyroscope** applies exponential moving average smoothing — the damping parameter controls how quickly it responds to changes. High damping (0.9) makes it sluggish but stable; low damping (0.1) makes it responsive but noisy.

The **heading estimator** fits a linear regression to the state history and extrapolates. It returns both a bearing and an R² confidence score.

The **anomaly detector** compares consecutive state vectors looking for reversals (all values flipped), stagnation (identical states), oscillation (values swinging between extremes), and unexpected transitions (jumps from -1 to +1). Sensitivity parameter controls the detection threshold.

## Known Limitations

- The 60° angular mapping is arbitrary — other mappings may make more sense for specific domains.
- The compass majority-vote can produce Neutral when approach and avoid are equally frequent, even if there's a clear oscillation.
- The gyroscope doesn't handle multi-dimensional state spaces — it works on scalar bearings only.
- Anomaly detection only compares consecutive states; multi-step patterns are not detected.
- The compass rose is limited to 2D (pairs of ternary values); higher-dimensional extensions don't exist yet.
- Linear regression for heading estimation assumes monotonic trends; it struggles with cyclical data.

## Use Cases

- **Robot navigation**: Track ternary steering decisions (left/straight/right) with heading and anomaly detection.
- **Sentiment analysis**: Monitor ternary sentiment (negative/neutral/positive) direction and detect sudden reversals.
- **Trading signals**: Track buy/hold/sell direction with gyroscope-stabilized orientation to filter noise.
- **Network monitoring**: Detect anomalous state transitions in ternary network health indicators.
- **Game AI**: Give agents navigational awareness in ternary decision spaces.

## Ecosystem Context

Part of the SuperInstance ternary ecosystem. Pairs naturally with `ternary-chronicle` (for recording navigational history) and `ternary-prophet` (for predicting future headings). The anomaly detector complements `ternary-dockyard` diagnostics.

## License

MIT
