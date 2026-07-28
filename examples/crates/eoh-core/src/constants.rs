//! Mathematical constants used throughout Eye of Horus.

/// The golden ratio φ = (1 + √5) / 2.
pub const PHI: f64 = 1.618_033_988_749_895;

/// π to full f64 precision.
pub const PI: f64 = std::f64::consts::PI;

/// φ / π — the fundamental addressing ratio used by the phi-pi model.
pub const PHI_PI_RATIO: f64 = PHI / PI;

/// Default expansion speed of a Higgs pulse in spatial units per tick.
pub const DEFAULT_PULSE_VELOCITY: f64 = 1.0;

/// Maximum allowed coordinate magnitude on any axis.
pub const MAX_COORD: f64 = 1_000_000.0;
