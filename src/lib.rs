#![forbid(unsafe_code)]

//! Orientation and direction in ternary state space.
//!
//! Provides navigational metaphors for understanding position and movement
//! within ternary {-1, 0, +1} state spaces: compass bearings, gyroscopic
//! stabilization, angular measurement, heading estimation, and anomaly detection.

use std::f64::consts::PI;

/// A ternary value: Negative (-1), Zero (0), or Positive (+1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ternary {
    Negative,
    Zero,
    Positive,
}

impl Ternary {
    pub fn value(self) -> i8 {
        match self {
            Ternary::Negative => -1,
            Ternary::Zero => 0,
            Ternary::Positive => 1,
        }
    }

    pub fn from_value(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Negative),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Positive),
            _ => None,
        }
    }
}

/// Ternary bearing: the fundamental direction in ternary space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bearing {
    /// Moving toward +1 (positive).
    Approach,
    /// Moving toward -1 (negative).
    Avoid,
    /// Holding at 0 (neutral).
    Neutral,
}

impl Bearing {
    pub fn from_delta(delta: i8) -> Self {
        if delta > 0 {
            Bearing::Approach
        } else if delta < 0 {
            Bearing::Avoid
        } else {
            Bearing::Neutral
        }
    }

    /// Convert to an angle in radians (0 = Neutral, PI/3 = Approach, -PI/3 = Avoid).
    pub fn to_angle(self) -> f64 {
        match self {
            Bearing::Approach => PI / 3.0,
            Bearing::Neutral => 0.0,
            Bearing::Avoid => -PI / 3.0,
        }
    }
}

// ─── Compass ─────────────────────────────────────────────────────────

/// Navigational instrument for ternary state space.
/// Tracks current bearing and orientation based on state transitions.
#[derive(Debug, Clone)]
pub struct Compass {
    /// Current bearing.
    bearing: Bearing,
    /// Confidence in the current bearing (0.0 to 1.0).
    confidence: f64,
    /// History of recent bearings for smoothing.
    history: Vec<Bearing>,
    /// Maximum history length.
    history_len: usize,
}

impl Compass {
    pub fn new(history_len: usize) -> Self {
        Compass {
            bearing: Bearing::Neutral,
            confidence: 1.0,
            history: Vec::new(),
            history_len,
        }
    }

    /// Update compass with a new state transition.
    pub fn update(&mut self, from: Ternary, to: Ternary) {
        let delta = to.value() - from.value();
        let new_bearing = Bearing::from_delta(delta);

        self.history.push(new_bearing);
        if self.history.len() > self.history_len {
            self.history.remove(0);
        }

        // Compute bearing by majority vote of history
        let approach_count = self.history.iter().filter(|&&b| b == Bearing::Approach).count();
        let avoid_count = self.history.iter().filter(|&&b| b == Bearing::Avoid).count();
        let neutral_count = self.history.iter().filter(|&&b| b == Bearing::Neutral).count();

        let (bearing, confidence) = if approach_count >= avoid_count && approach_count >= neutral_count {
            (Bearing::Approach, approach_count as f64 / self.history.len() as f64)
        } else if avoid_count >= approach_count && avoid_count >= neutral_count {
            (Bearing::Avoid, avoid_count as f64 / self.history.len() as f64)
        } else {
            (Bearing::Neutral, neutral_count as f64 / self.history.len() as f64)
        };

        self.bearing = bearing;
        self.confidence = confidence;
    }

    pub fn bearing(&self) -> Bearing {
        self.bearing
    }

    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    /// Reset the compass.
    pub fn reset(&mut self) {
        self.bearing = Bearing::Neutral;
        self.confidence = 1.0;
        self.history.clear();
    }
}

// ─── Gyroscope ───────────────────────────────────────────────────────

/// Maintains orientation during state transitions.
/// Provides stabilization by filtering out transient noise.
#[derive(Debug, Clone)]
pub struct Gyroscope {
    /// Current orientation angle (radians).
    orientation: f64,
    /// Smoothing factor (0.0 = no smoothing, 1.0 = maximum smoothing).
    damping: f64,
    /// Previous bearing for angular velocity.
    prev_bearing: Bearing,
}

impl Gyroscope {
    pub fn new(damping: f64) -> Self {
        Gyroscope {
            orientation: 0.0,
            damping: damping.clamp(0.0, 0.99),
            prev_bearing: Bearing::Neutral,
        }
    }

    /// Feed a new bearing; returns the stabilized orientation.
    pub fn stabilize(&mut self, bearing: Bearing) -> f64 {
        let target = bearing.to_angle();
        // Exponential moving average with damping
        let alpha = 1.0 - self.damping;
        self.orientation = self.damping * self.orientation + alpha * target;
        self.prev_bearing = bearing;
        self.orientation
    }

    /// Angular velocity (change since last update).
    pub fn angular_velocity(&self) -> f64 {
        self.orientation - self.prev_bearing.to_angle()
    }

    pub fn orientation(&self) -> f64 {
        self.orientation
    }

    /// Reset gyroscope to neutral.
    pub fn reset(&mut self) {
        self.orientation = 0.0;
        self.prev_bearing = Bearing::Neutral;
    }
}

// ─── Sextant ─────────────────────────────────────────────────────────

/// Measures angles between ternary states.
pub struct Sextant;

impl Sextant {
    /// Measure the angle between two ternary states in radians.
    /// States are mapped to unit vectors on a circle:
    ///   -1 → angle -PI/3
    ///    0 → angle 0
    ///   +1 → angle +PI/3
    pub fn angle_between(a: Ternary, b: Ternary) -> f64 {
        let angle_a = a.value() as f64 * PI / 3.0;
        let angle_b = b.value() as f64 * PI / 3.0;
        (angle_b - angle_a).abs()
    }

    /// Measure the angular distance across a sequence of states.
    pub fn total_traversal(states: &[Ternary]) -> f64 {
        if states.len() < 2 {
            return 0.0;
        }
        states.windows(2).map(|w| Self::angle_between(w[0], w[1])).sum()
    }

    /// Find the average angle between consecutive states.
    pub fn average_angle(states: &[Ternary]) -> f64 {
        if states.len() < 2 {
            return 0.0;
        }
        Self::total_traversal(states) / (states.len() - 1) as f64
    }

    /// Compute the "straightness" of a path (1.0 = perfectly straight, 0.0 = chaotic).
    pub fn straightness(states: &[Ternary]) -> f64 {
        if states.len() < 2 {
            return 1.0;
        }
        let net_displacement = (states.last().unwrap().value() - states.first().unwrap().value()).abs() as f64;
        let traversal = Self::total_traversal(states);
        if traversal < f64::EPSILON {
            return 1.0;
        }
        let net_angle = (states.last().unwrap().value() - states.first().unwrap().value()).abs() as f64 * PI / 3.0;
        (net_angle / traversal).min(1.0)
    }
}

// ─── HeadingEstimator ────────────────────────────────────────────────

/// Predicts future direction based on state history.
#[derive(Debug, Clone)]
pub struct HeadingEstimator {
    /// Window of recent states to consider.
    window_size: usize,
    /// Recent state history.
    history: Vec<Ternary>,
}

impl HeadingEstimator {
    pub fn new(window_size: usize) -> Self {
        HeadingEstimator {
            window_size: window_size.max(2),
            history: Vec::new(),
        }
    }

    /// Record a new state observation.
    pub fn observe(&mut self, state: Ternary) {
        self.history.push(state);
        if self.history.len() > self.window_size {
            self.history.remove(0);
        }
    }

    /// Estimate the current heading (predicted next bearing).
    pub fn estimate_heading(&self) -> (Bearing, f64) {
        if self.history.len() < 2 {
            return (Bearing::Neutral, 0.5);
        }

        // Compute trend: weighted linear regression on values
        let n = self.history.len() as f64;
        let mut sum_x = 0.0f64;
        let mut sum_y = 0.0f64;
        let mut sum_xy = 0.0f64;
        let mut sum_xx = 0.0f64;

        for (i, t) in self.history.iter().enumerate() {
            let x = i as f64;
            let y = t.value() as f64;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
        }

        let denom = n * sum_xx - sum_x * sum_x;
        if denom.abs() < f64::EPSILON {
            return (Bearing::Neutral, 1.0);
        }

        let slope = (n * sum_xy - sum_x * sum_y) / denom;
        let bearing = Bearing::from_delta(if slope > 0.1 { 1 } else if slope < -0.1 { -1 } else { 0 });

        // Confidence based on R²
        let mean_y = sum_y / n;
        let ss_tot: f64 = self.history.iter().map(|t| (t.value() as f64 - mean_y).powi(2)).sum();
        let intercept = (sum_y - slope * sum_x) / n;
        let ss_res: f64 = self.history.iter().enumerate()
            .map(|(i, t)| (t.value() as f64 - (intercept + slope * i as f64)).powi(2))
            .sum();
        let r_squared = if ss_tot > f64::EPSILON { 1.0 - ss_res / ss_tot } else { 0.0 };

        (bearing, r_squared.max(0.0).min(1.0))
    }

    /// Predict the state n steps ahead.
    pub fn predict(&self, steps: usize) -> Ternary {
        if self.history.is_empty() {
            return Ternary::Zero;
        }

        let n = self.history.len() as f64;
        let mut sum_x = 0.0f64;
        let mut sum_y = 0.0f64;
        let mut sum_xy = 0.0f64;
        let mut sum_xx = 0.0f64;

        for (i, t) in self.history.iter().enumerate() {
            let x = i as f64;
            let y = t.value() as f64;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
        }

        let denom = n * sum_xx - sum_x * sum_x;
        if denom.abs() < f64::EPSILON {
            return *self.history.last().unwrap();
        }

        let slope = (n * sum_xy - sum_x * sum_y) / denom;
        let intercept = (sum_y - slope * sum_x) / n;
        let predicted = intercept + slope * (self.history.len() + steps - 1) as f64;

        // Clamp to ternary range
        if predicted > 0.33 {
            Ternary::Positive
        } else if predicted < -0.33 {
            Ternary::Negative
        } else {
            Ternary::Zero
        }
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}

// ─── CompassRose ─────────────────────────────────────────────────────

/// The 8 principal directions of the ternary compass.
///
/// ```text
///        NNE (++)
///   NE (+0)   E (+-)
/// NW (0+)  ⬡  SE (-+)
///   W (0-)   SW (-0)
///        SSW (--)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryDirection {
    /// +1, +1 (strong approach)
    NorthNorthEast,
    /// +1, 0 (approach-neutral)
    NorthEast,
    /// +1, -1 (approach-avoid, mixed)
    East,
    /// 0, +1 (neutral-approach)
    NorthWest,
    /// 0, -1 (neutral-avoid)
    SouthEast,
    /// -1, +1 (avoid-approach, mixed)
    West,
    /// -1, 0 (avoid-neutral)
    SouthWest,
    /// -1, -1 (strong avoid)
    SouthSouthWest,
}

/// An 8-directional ternary compass rose.
pub struct CompassRose;

impl CompassRose {
    /// Get direction from a pair of ternary values (2D bearing).
    pub fn direction(a: Ternary, b: Ternary) -> Option<TernaryDirection> {
        match (a, b) {
            (Ternary::Positive, Ternary::Positive) => Some(TernaryDirection::NorthNorthEast),
            (Ternary::Positive, Ternary::Zero) => Some(TernaryDirection::NorthEast),
            (Ternary::Positive, Ternary::Negative) => Some(TernaryDirection::East),
            (Ternary::Zero, Ternary::Positive) => Some(TernaryDirection::NorthWest),
            (Ternary::Zero, Ternary::Negative) => Some(TernaryDirection::SouthEast),
            (Ternary::Negative, Ternary::Positive) => Some(TernaryDirection::West),
            (Ternary::Negative, Ternary::Zero) => Some(TernaryDirection::SouthWest),
            (Ternary::Negative, Ternary::Negative) => Some(TernaryDirection::SouthSouthWest),
            (Ternary::Zero, Ternary::Zero) => None, // no direction
        }
    }

    /// Get all 8 directions.
    pub fn all_directions() -> Vec<TernaryDirection> {
        vec![
            TernaryDirection::NorthNorthEast,
            TernaryDirection::NorthEast,
            TernaryDirection::East,
            TernaryDirection::NorthWest,
            TernaryDirection::SouthEast,
            TernaryDirection::West,
            TernaryDirection::SouthWest,
            TernaryDirection::SouthSouthWest,
        ]
    }

    /// Get the opposite direction.
    pub fn opposite(dir: TernaryDirection) -> TernaryDirection {
        match dir {
            TernaryDirection::NorthNorthEast => TernaryDirection::SouthSouthWest,
            TernaryDirection::NorthEast => TernaryDirection::SouthWest,
            TernaryDirection::East => TernaryDirection::West,
            TernaryDirection::NorthWest => TernaryDirection::SouthEast,
            TernaryDirection::SouthEast => TernaryDirection::NorthWest,
            TernaryDirection::West => TernaryDirection::East,
            TernaryDirection::SouthWest => TernaryDirection::NorthEast,
            TernaryDirection::SouthSouthWest => TernaryDirection::NorthNorthEast,
        }
    }

    /// Convert direction to an angle in radians.
    pub fn to_angle(dir: TernaryDirection) -> f64 {
        match dir {
            TernaryDirection::NorthNorthEast => PI / 4.0,
            TernaryDirection::NorthEast => PI / 4.0 + PI / 8.0,
            TernaryDirection::East => PI / 2.0,
            TernaryDirection::NorthWest => -PI / 8.0,
            TernaryDirection::SouthEast => 3.0 * PI / 4.0,
            TernaryDirection::West => -PI / 2.0,
            TernaryDirection::SouthWest => -(PI / 4.0 + PI / 8.0),
            TernaryDirection::SouthSouthWest => -PI / 4.0,
        }
    }
}

// ─── MagneticAnomaly ─────────────────────────────────────────────────

/// Represents a distortion detected in ternary state space.
#[derive(Debug, Clone)]
pub struct MagneticAnomaly {
    /// Position in state space where anomaly was detected.
    pub position: Vec<Ternary>,
    /// Strength of the anomaly (0.0 to 1.0).
    pub strength: f64,
    /// What kind of anomaly was detected.
    pub kind: AnomalyKind,
}

/// Classification of ternary state space anomalies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnomalyKind {
    /// Sudden reversal of direction.
    Reversal,
    /// Oscillating between states.
    Oscillation,
    /// State frozen at one value.
    Stagnation,
    /// Transition that violates expected patterns.
    UnexpectedTransition,
}

/// Detects distortions and anomalies in ternary state space.
pub struct AnomalyDetector {
    /// Sensitivity threshold (0.0 = detect everything, 1.0 = detect nothing).
    sensitivity: f64,
    /// Previous state for comparison.
    prev_state: Option<Vec<Ternary>>,
}

impl AnomalyDetector {
    pub fn new(sensitivity: f64) -> Self {
        AnomalyDetector {
            sensitivity: sensitivity.clamp(0.0, 1.0),
            prev_state: None,
        }
    }

    /// Scan a state vector for anomalies.
    pub fn scan(&mut self, state: &[Ternary]) -> Vec<MagneticAnomaly> {
        let mut anomalies = Vec::new();

        if let Some(ref prev) = self.prev_state {
            if prev.len() == state.len() {
                // Check for reversals: every value flipped sign
                let all_reversed = state.iter().zip(prev.iter())
                    .all(|(a, b)| a.value() == -b.value() && a.value() != 0);
                if all_reversed && state.len() > 0 {
                    let strength = 1.0 - self.sensitivity;
                    if strength > 0.2 {
                        anomalies.push(MagneticAnomaly {
                            position: state.to_vec(),
                            strength,
                            kind: AnomalyKind::Reversal,
                        });
                    }
                }

                // Check for stagnation: identical to previous
                if state == prev.as_slice() {
                    let strength = 0.5 * (1.0 - self.sensitivity);
                    if strength > 0.1 {
                        anomalies.push(MagneticAnomaly {
                            position: state.to_vec(),
                            strength,
                            kind: AnomalyKind::Stagnation,
                        });
                    }
                }

                // Check for oscillation: current matches state before previous
                // (We only have one previous, so partial check on individual elements)
                let oscillating_elements: Vec<usize> = state.iter().enumerate()
                    .filter(|(i, s)| {
                        let delta = s.value() - prev[*i].value();
                        delta != 0 && delta.abs() == 2 // swung from -1 to +1 or vice versa
                    })
                    .map(|(i, _)| i)
                    .collect();
                if oscillating_elements.len() > state.len() / 2 {
                    let strength = 0.7 * (1.0 - self.sensitivity);
                    anomalies.push(MagneticAnomaly {
                        position: state.to_vec(),
                        strength,
                        kind: AnomalyKind::Oscillation,
                    });
                }

                // Check for unexpected transitions (jump from -1 to +1 or vice versa)
                for (i, (curr, prev_val)) in state.iter().zip(prev.iter()).enumerate() {
                    if (curr.value() - prev_val.value()).abs() == 2 {
                        let strength = 0.6 * (1.0 - self.sensitivity);
                        if strength > 0.15 {
                            anomalies.push(MagneticAnomaly {
                                position: state.to_vec(),
                                strength,
                                kind: AnomalyKind::UnexpectedTransition,
                            });
                            break;
                        }
                    }
                }
            }
        }

        self.prev_state = Some(state.to_vec());

        // Filter by sensitivity
        anomalies.into_iter().filter(|a| a.strength > (1.0 - self.sensitivity) * 0.1).collect()
    }

    /// Reset detector state.
    pub fn reset(&mut self) {
        self.prev_state = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_values() {
        assert_eq!(Ternary::Negative.value(), -1);
        assert_eq!(Ternary::Zero.value(), 0);
        assert_eq!(Ternary::Positive.value(), 1);
    }

    #[test]
    fn test_bearing_from_delta() {
        assert_eq!(Bearing::from_delta(1), Bearing::Approach);
        assert_eq!(Bearing::from_delta(-1), Bearing::Avoid);
        assert_eq!(Bearing::from_delta(0), Bearing::Neutral);
    }

    #[test]
    fn test_compass_update() {
        let mut compass = Compass::new(5);
        compass.update(Ternary::Zero, Ternary::Positive);
        assert_eq!(compass.bearing(), Bearing::Approach);
    }

    #[test]
    fn test_compass_smoothing() {
        let mut compass = Compass::new(3);
        compass.update(Ternary::Zero, Ternary::Positive);
        compass.update(Ternary::Positive, Ternary::Positive); // Neutral delta
        compass.update(Ternary::Positive, Ternary::Positive); // Neutral again
        // Majority neutral
        assert_eq!(compass.bearing(), Bearing::Neutral);
    }

    #[test]
    fn test_compass_confidence() {
        let mut compass = Compass::new(3);
        compass.update(Ternary::Zero, Ternary::Positive);
        assert!(compass.confidence() > 0.0);
    }

    #[test]
    fn test_compass_reset() {
        let mut compass = Compass::new(5);
        compass.update(Ternary::Zero, Ternary::Positive);
        compass.reset();
        assert_eq!(compass.bearing(), Bearing::Neutral);
    }

    #[test]
    fn test_gyroscope_stabilize() {
        let mut gyro = Gyroscope::new(0.5);
        let angle = gyro.stabilize(Bearing::Approach);
        assert!(angle > 0.0);
    }

    #[test]
    fn test_gyroscope_damping() {
        let mut high_damp = Gyroscope::new(0.9);
        let mut low_damp = Gyroscope::new(0.1);

        high_damp.stabilize(Bearing::Approach);
        low_damp.stabilize(Bearing::Approach);

        // High damping should resist change more (orientation closer to 0)
        assert!(high_damp.orientation() < low_damp.orientation());
    }

    #[test]
    fn test_gyroscope_reset() {
        let mut gyro = Gyroscope::new(0.5);
        gyro.stabilize(Bearing::Avoid);
        gyro.reset();
        assert!((gyro.orientation() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sextant_angle_between_same() {
        let angle = Sextant::angle_between(Ternary::Zero, Ternary::Zero);
        assert!((angle - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sextant_angle_between_opposite() {
        let angle = Sextant::angle_between(Ternary::Negative, Ternary::Positive);
        assert!(angle > 1.0);
    }

    #[test]
    fn test_sextant_total_traversal() {
        let states = vec![Ternary::Zero, Ternary::Positive, Ternary::Zero];
        let traversal = Sextant::total_traversal(&states);
        assert!(traversal > 0.0);
    }

    #[test]
    fn test_sextant_straightness_perfect() {
        let states = vec![Ternary::Zero, Ternary::Positive, Ternary::Positive, Ternary::Positive];
        let s = Sextant::straightness(&states);
        assert!(s > 0.9);
    }

    #[test]
    fn test_sextant_straightness_chaotic() {
        let states = vec![Ternary::Positive, Ternary::Negative, Ternary::Positive, Ternary::Negative];
        let s = Sextant::straightness(&states);
        assert!(s < 0.5);
    }

    #[test]
    fn test_heading_estimator_trend() {
        let mut estimator = HeadingEstimator::new(10);
        estimator.observe(Ternary::Negative);
        estimator.observe(Ternary::Zero);
        estimator.observe(Ternary::Positive);
        let (bearing, _conf) = estimator.estimate_heading();
        assert_eq!(bearing, Bearing::Approach);
    }

    #[test]
    fn test_heading_estimator_predict() {
        let mut estimator = HeadingEstimator::new(10);
        estimator.observe(Ternary::Negative);
        estimator.observe(Ternary::Negative);
        estimator.observe(Ternary::Zero);
        let predicted = estimator.predict(2);
        assert_eq!(predicted, Ternary::Positive);
    }

    #[test]
    fn test_heading_estimator_short_history() {
        let mut estimator = HeadingEstimator::new(5);
        estimator.observe(Ternary::Positive);
        let (bearing, _) = estimator.estimate_heading();
        assert_eq!(bearing, Bearing::Neutral); // insufficient data
    }

    #[test]
    fn test_compass_rose_all_directions() {
        assert_eq!(CompassRose::all_directions().len(), 8);
    }

    #[test]
    fn test_compass_rose_direction() {
        let dir = CompassRose::direction(Ternary::Positive, Ternary::Positive);
        assert_eq!(dir, Some(TernaryDirection::NorthNorthEast));
    }

    #[test]
    fn test_compass_rose_no_direction() {
        let dir = CompassRose::direction(Ternary::Zero, Ternary::Zero);
        assert_eq!(dir, None);
    }

    #[test]
    fn test_compass_rose_opposite() {
        let opp = CompassRose::opposite(TernaryDirection::NorthNorthEast);
        assert_eq!(opp, TernaryDirection::SouthSouthWest);
    }

    #[test]
    fn test_compass_rose_opposite_roundtrip() {
        for dir in CompassRose::all_directions() {
            assert_eq!(CompassRose::opposite(CompassRose::opposite(dir)), dir);
        }
    }

    #[test]
    fn test_anomaly_detector_reversal() {
        let mut detector = AnomalyDetector::new(0.0);
        detector.scan(&[Ternary::Positive, Ternary::Positive]);
        let anomalies = detector.scan(&[Ternary::Negative, Ternary::Negative]);
        assert!(anomalies.iter().any(|a| a.kind == AnomalyKind::Reversal));
    }

    #[test]
    fn test_anomaly_detector_stagnation() {
        let mut detector = AnomalyDetector::new(0.0);
        detector.scan(&[Ternary::Positive, Ternary::Zero]);
        let anomalies = detector.scan(&[Ternary::Positive, Ternary::Zero]);
        assert!(anomalies.iter().any(|a| a.kind == AnomalyKind::Stagnation));
    }

    #[test]
    fn test_anomaly_detector_unexpected_transition() {
        let mut detector = AnomalyDetector::new(0.0);
        detector.scan(&[Ternary::Negative]);
        let anomalies = detector.scan(&[Ternary::Positive]);
        assert!(anomalies.iter().any(|a| a.kind == AnomalyKind::UnexpectedTransition));
    }

    #[test]
    fn test_anomaly_detector_high_sensitivity() {
        let mut detector = AnomalyDetector::new(0.99);
        detector.scan(&[Ternary::Positive]);
        let anomalies = detector.scan(&[Ternary::Negative]);
        // High sensitivity threshold filters most anomalies
        // The unexpected transition has strength 0.6 * (1-0.99) = 0.006
        // which is below threshold 0.1 * 0.01 = 0.001 — but just barely
        // Let's verify strength is low
        let max_strength = anomalies.iter().map(|a| a.strength).fold(0.0f64, f64::max);
        assert!(max_strength < 0.1);
    }

    #[test]
    fn test_anomaly_detector_reset() {
        let mut detector = AnomalyDetector::new(0.0);
        detector.scan(&[Ternary::Positive]);
        detector.reset();
        let anomalies = detector.scan(&[Ternary::Negative]);
        // No previous state to compare, should be empty
        assert!(anomalies.is_empty());
    }
}
