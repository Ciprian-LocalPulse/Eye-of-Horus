//! Core utilities for the Eye of Horus research implementation.
//!
//! This crate is intentionally small. It does not implement the full language.

pub mod parser;

/// Golden ratio approximation used by the draft phi-pi address function.
pub const PHI: f64 = 1.618_033_988_749_895;

/// Pi, re-exported for language-design experiments.
pub const PI: f64 = std::f64::consts::PI;

/// Three-dimensional point in draft language space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3 {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Z coordinate.
    pub z: f64,
}

impl Point3 {
    /// Creates a point.
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Computes Euclidean distance to another point.
    #[must_use]
    pub fn distance_to(self, other: Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Computes the draft phi-pi address for the nth declared entity.
///
/// This function is deterministic and public. It is not a security primitive.
#[must_use]
pub fn phi_pi_address(n: u64) -> u64 {
    ((n as f64) * PHI * PI).floor() as u64
}

/// Public status of the current language implementation.
#[must_use]
pub fn implementation_status() -> &'static str {
    "research draft: specification scaffold and minimal Rust utilities only"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phi_pi_address_is_monotonic_for_initial_range() {
        let mut previous = phi_pi_address(0);
        for n in 1..100 {
            let current = phi_pi_address(n);
            assert!(current >= previous);
            previous = current;
        }
    }

    #[test]
    fn distance_uses_euclidean_geometry() {
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Point3::new(3.0, 4.0, 12.0);
        assert_eq!(a.distance_to(b), 13.0);
    }
}
