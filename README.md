# ternary-compass

Orientation and direction in ternary state space — compass bearings, gyroscopic stabilization, angular measurement, heading estimation, and anomaly detection for {-1, 0, +1} navigation.

## Background

Navigation requires knowing where you are, which way you're heading, and whether something unexpected has happened. In continuous spaces, we use compasses, gyroscopes, and magnetometers. But what does "direction" mean in a three-valued discrete space?

Ternary state spaces appear everywhere in the Oxide stack: signal classification (negative/neutral/positive), decision outcomes (reject/abstain/approve), and process states (error/idle/active). The `ternary-compass` crate treats these ternary streams as navigational data, providing instruments to track bearing, stabilize orientation, predict heading, and detect anomalies — all within the {-1, 0, +1} domain.

The metaphor is deliberate: just as a magnetic compass smooths out local magnetic noise to find true north, a ternary compass smooths out transient state fluctuations to reveal the underlying directional trend.

## How It Works

### Ternary Bearing

The fundamental direction in ternary space. Three possible bearings:

| Bearing   | Delta | Angle    | Interpretation    |
|-----------|-------|----------|-------------------|
| Approach  | > 0   | +π/3     | Moving toward +1  |
| Neutral   | = 0   | 0        | Holding at current|
| Avoid     | < 0   | −π/3     | Moving toward −1  |

Bearings are derived from state transitions: the delta between consecutive ternary values maps to a directional classification.

### Compass (Majority-Vote Smoothing)

The `Compass` tracks a history of recent bearings and computes the current direction by majority vote. A configurable history length controls smoothing: longer histories produce more stable bearings at the cost of responsiveness. Confidence is the fraction of history agreeing with the majority bearing.

### Gyroscope (Exponential Damping)

The `Gyroscope` maintains a continuous orientation angle using exponential moving average damping. Unlike the discrete compass, the gyroscope produces smooth angular output that filters transient noise. The damping factor (0.0–0.99) controls how quickly the orientation responds to bearing changes.

### Sextant (Angular Measurement)

The `Sextant` measures angular distances between ternary states, treating {-1, 0, +1} as points on a unit circle at angles {−π/3, 0, +π/3}. It computes:

- **Angle between** any two states
- **Total traversal** across a sequence
- **Straightness** — how directly a path travels (1.0 = straight, 0.0 = chaotic)

### HeadingEstimator (Linear Regression Prediction)

Predicts future direction using weighted linear regression on recent state history. Returns both a predicted bearing and an R² confidence score. The `predict` method extrapolates n steps ahead, clamping the result back to ternary range.

### Compass Rose (2D Ternary Directions)

An 8-directional compass for pairs of ternary values, analogous to the traditional 16-point wind rose. Each combination of two ternary values maps to a unique direction, with `Zero/Zero` being the null direction (no movement).

### Anomaly Detector

Scans ternary state vectors for four classes of anomalies:

- **Reversal** — all values flip sign simultaneously
- **Stagnation** — state identical to previous observation
- **Oscillation** — majority of elements swing between extremes
- **Unexpected transition** — any element jumps from −1 to +1 or vice versa

Sensitivity is configurable: higher thresholds filter weaker anomalies.

## Experimental Results

- **Compass smoothing is robust.** With history length 5, a single transient bearing change doesn't alter the compass reading. Two consecutive changes shift the majority. Three are needed for a full bearing reversal — a useful hysteresis property.
- **Gyroscope damping at 0.5 responds within 2-3 steps.** At 0.9, the orientation changes slowly enough that 10+ steps of consistent bearing are needed for convergence.
- **Sextant straightness distinguishes trends from noise.** A monotonically increasing sequence [Neg, Zero, Pos] scores 1.0. An alternating [Pos, Neg, Pos, Neg] scores below 0.5.
- **Heading prediction works for linear trends.** A sequence [Neg, Neg, Zero, Pos] correctly predicts Pos two steps ahead. Chaotic sequences produce low R² and default to Neutral bearing.

## Impact

`ternary-compass` demonstrates that navigational metaphors — bearings, headings, gyroscopic stabilization — remain meaningful and useful even in a three-valued discrete space. The instruments provide genuine signal processing: smoothing noise, detecting anomalies, and predicting trends from ternary data streams.

The crate establishes that ternary state spaces have inherent geometry: the mapping to angles on a circle is natural (not arbitrary), and the resulting angular measurements carry real information about state trajectory dynamics.

## Use Cases

1. **Agent fleet monitoring** — Track the directional trend of distributed ternary signals (e.g., health scores across a cluster) and detect when the fleet unexpectedly reverses or stagnates.
2. **Real-time decision smoothing** — Use compass/gyroscope to filter jittery ternary decisions (buy/hold/sell, approve/abstain/reject) into stable directional signals.
3. **Anomaly detection in control systems** — Detect reversals, oscillations, and stagnation in ternary-valued sensor outputs or process state machines.
4. **Musical direction tracking** — Track the harmonic "direction" of a ternary music stream (tension/neutral/resolution) to build responsive accompaniment systems.

## Open Questions

1. **3D ternary navigation.** The compass rose handles 2D (pairs of ternary values). What would a full 3D ternary compass look like, and would it correspond to known crystallographic symmetry groups?
2. **Kalman filtering for ternary.** Could a Kalman-style filter be adapted to ternary state estimation, combining the compass (bearing) and gyroscope (orientation) into a unified estimator?
3. **Anomaly classification accuracy.** The current detector uses heuristic thresholds. Could a learned model (even a simple ternary perceptron) improve anomaly detection accuracy?

## Connection to Oxide Stack

`ternary-compass` provides navigational primitives consumed across the stack: `ternary-tidelight` uses `TideClock` timing with compass bearings to schedule fleet synchronization, `ternary-ear` uses heading estimation to predict rhythmic patterns, and `ternary-rhythm` uses the anomaly detector to identify syncopation and rhythmic irregularities. The angular measurement framework connects to `ternary-color`'s hue-based temperature classification.
