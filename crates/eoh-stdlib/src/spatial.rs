//! Spatial query utilities — nearest-neighbour, range search, etc.

use eoh_core::coordinates::Coord3D;

/// Find the vertex in `candidates` closest to `query`.
///
/// Returns `None` if `candidates` is empty.
pub fn nearest_vertex<'a>(query: &Coord3D, candidates: &'a [Coord3D]) -> Option<&'a Coord3D> {
    candidates
        .iter()
        .min_by(|a, b| {
            query.distance(a)
                .partial_cmp(&query.distance(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

/// Collect all vertices within `radius` distance of `query`.
pub fn vertices_within_radius<'a>(query: &Coord3D, candidates: &'a [Coord3D], radius: f64) -> Vec<&'a Coord3D> {
    candidates.iter().filter(|c| query.distance(c) <= radius).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coord(x: f64, y: f64, z: f64) -> Coord3D { Coord3D { x, y, z } }

    #[test]
    fn nearest_returns_closest() {
        let candidates = vec![coord(10.0, 0.0, 0.0), coord(1.0, 0.0, 0.0), coord(5.0, 0.0, 0.0)];
        let q = coord(0.0, 0.0, 0.0);
        let n = nearest_vertex(&q, &candidates).unwrap();
        assert!((n.x - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn radius_query() {
        let candidates = vec![coord(1.0, 0.0, 0.0), coord(5.0, 0.0, 0.0)];
        let q = coord(0.0, 0.0, 0.0);
        let within = vertices_within_radius(&q, &candidates, 2.0);
        assert_eq!(within.len(), 1);
    }
}
