//! Mathematical utility functions.

use eoh_core::constants::PHI;

/// Linear interpolation between `a` and `b` at parameter `t ∈ [0, 1]`.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

/// Clamp `v` to the closed interval `[lo, hi]`.
pub fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    v.max(lo).min(hi)
}

/// The golden angle in radians: `2π(2 - φ)`.
///
/// Used for optimal Fibonacci spiral point distributions on a sphere.
pub fn golden_angle() -> f64 {
    2.0 * std::f64::consts::PI * (2.0 - PHI)
}

/// Map integer index `n` to a point on the unit sphere using the Fibonacci
/// lattice, using the golden angle for angular spacing.
pub fn fibonacci_sphere(n: u32, total: u32) -> (f64, f64, f64) {
    let theta = golden_angle() * n as f64;
    let phi = (1.0 - 2.0 * (n as f64 + 0.5) / total as f64).acos();
    let x = phi.sin() * theta.cos();
    let y = phi.sin() * theta.sin();
    let z = phi.cos();
    (x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_midpoint() {
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn clamp_bounds() {
        assert_eq!(clamp(-5.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(5.0, 0.0, 1.0), 1.0);
        assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
    }

    #[test]
    fn fibonacci_sphere_unit_length() {
        let (x, y, z) = fibonacci_sphere(7, 100);
        let len = (x * x + y * y + z * z).sqrt();
        assert!((len - 1.0).abs() < 1e-10);
    }
}
