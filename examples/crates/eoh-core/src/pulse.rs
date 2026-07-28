//! The Higgs-pulse activation model.
//!
//! A [`Pulse`] propagates outward from an origin in 3-D space at a configurable
//! velocity. Any [`Shape`] whose bounding volume intersects the pulse wavefront
//! at simulation tick *t* is *activated* for that tick and scheduled for
//! execution by the VM.

use crate::{constants::DEFAULT_PULSE_VELOCITY, coordinates::Coord3D};
use serde::{Deserialize, Serialize};

/// A directional pulse vector — direction + implied magnitude.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PulseVector {
    /// X component.
    pub dx: f64,
    /// Y component.
    pub dy: f64,
    /// Z component.
    pub dz: f64,
}

impl PulseVector {
    /// Construct from raw components.
    pub fn new(dx: f64, dy: f64, dz: f64) -> Self { Self { dx, dy, dz } }

    /// Magnitude of this vector.
    pub fn magnitude(&self) -> f64 {
        (self.dx * self.dx + self.dy * self.dy + self.dz * self.dz).sqrt()
    }

    /// Unit vector in the same direction. Returns `None` for the zero vector.
    pub fn normalised(&self) -> Option<Self> {
        let m = self.magnitude();
        if m == 0.0 { return None; }
        Some(Self { dx: self.dx / m, dy: self.dy / m, dz: self.dz / m })
    }

    /// Isotropic (omnidirectional) pulse — zero vector.
    pub const ISOTROPIC: Self = Self { dx: 0.0, dy: 0.0, dz: 0.0 };
}

/// A Higgs-pulse originating at a spatial point and expanding at a set velocity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pulse {
    /// Point from which the pulse radiates.
    pub origin: Coord3D,
    /// Expansion speed in spatial units per simulation tick.
    pub velocity: f64,
    /// Optional directional bias (zero vector → isotropic expansion).
    pub direction: PulseVector,
    /// Simulation tick at which this pulse was emitted.
    pub birth_tick: u64,
}

impl Pulse {
    /// Create an isotropic pulse at the given origin with default velocity.
    pub fn isotropic(origin: Coord3D, birth_tick: u64) -> Self {
        Self {
            origin,
            velocity: DEFAULT_PULSE_VELOCITY,
            direction: PulseVector::ISOTROPIC,
            birth_tick,
        }
    }

    /// Wavefront radius at the given simulation tick.
    pub fn radius_at(&self, tick: u64) -> f64 {
        tick.saturating_sub(self.birth_tick) as f64 * self.velocity
    }

    /// `true` if `point` falls within the wavefront at `tick`.
    pub fn activates(&self, point: &Coord3D, tick: u64) -> bool {
        self.origin.distance(point) <= self.radius_at(tick)
    }
}

/// A superposition of one or more pulses defining which shapes are active during
/// a given simulation tick.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivationField {
    /// Active pulses in this field.
    pub pulses: Vec<Pulse>,
}

impl ActivationField {
    /// Add a pulse to the field.
    pub fn add(&mut self, pulse: Pulse) { self.pulses.push(pulse); }

    /// `true` if *any* pulse activates `point` at `tick`.
    pub fn is_active(&self, point: &Coord3D, tick: u64) -> bool {
        self.pulses.iter().any(|p| p.activates(point, tick))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radius_grows_linearly() {
        let p = Pulse::isotropic(Coord3D::ORIGIN, 0);
        assert_eq!(p.radius_at(0), 0.0);
        assert!((p.radius_at(5) - 5.0 * DEFAULT_PULSE_VELOCITY).abs() < f64::EPSILON);
    }

    #[test]
    fn activates_point_on_wavefront() {
        let p = Pulse::isotropic(Coord3D::ORIGIN, 0);
        let pt = Coord3D::new(1.0, 0.0, 0.0).unwrap();
        assert!(p.activates(&pt, 1));
        assert!(!p.activates(&pt, 0));
    }

    #[test]
    fn activation_field_union_semantics() {
        let mut field = ActivationField::default();
        field.add(Pulse::isotropic(Coord3D::ORIGIN, 0));
        let far = Coord3D::new(100.0, 0.0, 0.0).unwrap();
        assert!(!field.is_active(&far, 1));
        assert!(field.is_active(&far, 101));
    }
}
