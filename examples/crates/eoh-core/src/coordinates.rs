//! Three-dimensional coordinate types and the phi-pi addressing model.

use crate::{
    constants::{MAX_COORD, PHI, PI},
    error::{EohError, EohResult},
};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

/// A point in Eye of Horus 3-D coordinate space.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coord3D {
    /// X axis.
    pub x: f64,
    /// Y axis.
    pub y: f64,
    /// Z axis.
    pub z: f64,
}

impl Coord3D {
    /// Construct and validate a coordinate triple.
    pub fn new(x: f64, y: f64, z: f64) -> EohResult<Self> {
        for (name, v) in [("x", x), ("y", y), ("z", z)] {
            if v.is_nan() || v.is_infinite() {
                return Err(EohError::Geometry(format!(
                    "{name} must be finite, got {v}"
                )));
            }
            if v.abs() > MAX_COORD {
                return Err(EohError::Geometry(format!(
                    "{name} = {v} exceeds MAX_COORD ({MAX_COORD})"
                )));
            }
        }
        Ok(Self { x, y, z })
    }

    /// The canonical spatial origin.
    pub const ORIGIN: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    /// Euclidean distance to another coordinate.
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Midpoint between two coordinates.
    pub fn midpoint(&self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
            z: (self.z + other.z) / 2.0,
        }
    }
}

impl Eq for Coord3D {}

impl std::hash::Hash for Coord3D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        OrderedFloat(self.x).hash(state);
        OrderedFloat(self.y).hash(state);
        OrderedFloat(self.z).hash(state);
    }
}

impl std::fmt::Display for Coord3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.6}, {:.6}, {:.6})", self.x, self.y, self.z)
    }
}

// ── Phi-Pi address ──────────────────────────────────────────────────────────

/// A phi-pi address encodes a spatial location using the golden-ratio/pi ratio
/// as the addressing granularity.
///
/// **Research note:** this model is deterministic and public. It is intended as
/// a research and teaching construct and provides no cryptographic properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhiPiAddress {
    /// Phi-lattice index along X.
    pub ix: i64,
    /// Phi-lattice index along Y.
    pub iy: i64,
    /// Phi-lattice index along Z.
    pub iz: i64,
}

impl PhiPiAddress {
    /// Convert a continuous coordinate to the nearest phi-pi lattice address.
    pub fn from_coord(c: &Coord3D) -> Self {
        let quantum = PHI / PI;
        Self {
            ix: (c.x / quantum).round() as i64,
            iy: (c.y / quantum).round() as i64,
            iz: (c.z / quantum).round() as i64,
        }
    }

    /// Recover the lattice-centre coordinate for this address.
    pub fn to_coord(&self) -> Coord3D {
        let quantum = PHI / PI;
        Coord3D {
            x: self.ix as f64 * quantum,
            y: self.iy as f64 * quantum,
            z: self.iz as f64 * quantum,
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_is_zero() {
        assert_eq!(Coord3D::ORIGIN, Coord3D { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn distance_symmetric() {
        let a = Coord3D::new(1.0, 0.0, 0.0).unwrap();
        let b = Coord3D::new(0.0, 1.0, 0.0).unwrap();
        assert!((a.distance(&b) - b.distance(&a)).abs() < f64::EPSILON);
    }

    #[test]
    fn midpoint_correct() {
        let a = Coord3D::new(0.0, 0.0, 0.0).unwrap();
        let b = Coord3D::new(2.0, 2.0, 2.0).unwrap();
        let m = a.midpoint(&b);
        assert!((m.x - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_nan() {
        assert!(Coord3D::new(f64::NAN, 0.0, 0.0).is_err());
    }

    #[test]
    fn rejects_infinity() {
        assert!(Coord3D::new(f64::INFINITY, 0.0, 0.0).is_err());
    }

    #[test]
    fn phi_pi_roundtrip_within_one_quantum() {
        let c = Coord3D::new(1.0, 2.0, 3.0).unwrap();
        let addr = PhiPiAddress::from_coord(&c);
        let recovered = addr.to_coord();
        assert!(c.distance(&recovered) < PHI / PI + f64::EPSILON);
    }
}
