//! Spatial field — the primary storage model for Eye of Horus.
//!
//! A [`SpatialField`] maps phi-pi lattice addresses to arbitrary values. In the
//! VM this replaces the conventional heap/stack distinction; values live at
//! geometric locations and are accessed by address rather than pointer.

use crate::coordinates::PhiPiAddress;
use std::collections::HashMap;

/// A typed spatial field mapping addresses to values of type `V`.
#[derive(Debug, Default)]
pub struct SpatialField<V> {
    cells: HashMap<PhiPiAddress, V>,
}

impl<V: Clone> SpatialField<V> {
    /// Construct an empty field.
    pub fn new() -> Self { Self { cells: HashMap::new() } }

    /// Write a value at the given address.
    pub fn write(&mut self, addr: PhiPiAddress, value: V) {
        self.cells.insert(addr, value);
    }

    /// Read a value from the given address. Returns `None` if unwritten.
    pub fn read(&self, addr: &PhiPiAddress) -> Option<&V> {
        self.cells.get(addr)
    }

    /// Remove and return the value at `addr`.
    pub fn consume(&mut self, addr: &PhiPiAddress) -> Option<V> {
        self.cells.remove(addr)
    }

    /// Number of occupied cells.
    pub fn occupied(&self) -> usize { self.cells.len() }

    /// Iterate over all occupied cells.
    pub fn iter(&self) -> impl Iterator<Item = (&PhiPiAddress, &V)> {
        self.cells.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinates::Coord3D;

    fn addr_at(x: f64, y: f64, z: f64) -> PhiPiAddress {
        PhiPiAddress::from_coord(&Coord3D::new(x, y, z).unwrap())
    }

    #[test]
    fn write_and_read() {
        let mut f: SpatialField<i32> = SpatialField::new();
        let a = addr_at(1.0, 0.0, 0.0);
        f.write(a, 42);
        assert_eq!(f.read(&a), Some(&42));
    }

    #[test]
    fn consume_removes_value() {
        let mut f: SpatialField<i32> = SpatialField::new();
        let a = addr_at(2.0, 0.0, 0.0);
        f.write(a, 7);
        assert_eq!(f.consume(&a), Some(7));
        assert_eq!(f.read(&a), None);
    }
}
