//! Geometric utility functions for Eye of Horus programs.

use eoh_core::coordinates::Coord3D;

/// Compute the centroid (arithmetic mean position) of a slice of coordinates.
///
/// Returns `None` if the slice is empty.
pub fn centroid(points: &[Coord3D]) -> Option<Coord3D> {
    if points.is_empty() { return None; }
    let n = points.len() as f64;
    Some(Coord3D {
        x: points.iter().map(|p| p.x).sum::<f64>() / n,
        y: points.iter().map(|p| p.y).sum::<f64>() / n,
        z: points.iter().map(|p| p.z).sum::<f64>() / n,
    })
}

/// Return the axis-aligned bounding box `(min, max)` of a point set.
///
/// Returns `None` if the slice is empty.
pub fn bounding_box(points: &[Coord3D]) -> Option<(Coord3D, Coord3D)> {
    if points.is_empty() { return None; }
    let mut min = points[0];
    let mut max = points[0];
    for p in points.iter().skip(1) {
        min.x = min.x.min(p.x);
        min.y = min.y.min(p.y);
        min.z = min.z.min(p.z);
        max.x = max.x.max(p.x);
        max.y = max.y.max(p.y);
        max.z = max.z.max(p.z);
    }
    Some((min, max))
}

/// Compute the surface area of a tetrahedron given its four vertex coordinates.
///
/// Uses the cross-product formula for each of the four triangular faces.
pub fn surface_area_tetra(a: &Coord3D, b: &Coord3D, c: &Coord3D, d: &Coord3D) -> f64 {
    triangle_area(a, b, c)
        + triangle_area(a, b, d)
        + triangle_area(a, c, d)
        + triangle_area(b, c, d)
}

fn triangle_area(p0: &Coord3D, p1: &Coord3D, p2: &Coord3D) -> f64 {
    let ab = (p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
    let ac = (p2.x - p0.x, p2.y - p0.y, p2.z - p0.z);
    let cross = (
        ab.1 * ac.2 - ab.2 * ac.1,
        ab.2 * ac.0 - ab.0 * ac.2,
        ab.0 * ac.1 - ab.1 * ac.0,
    );
    0.5 * (cross.0 * cross.0 + cross.1 * cross.1 + cross.2 * cross.2).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centroid_single_point() {
        let pts = vec![Coord3D { x: 3.0, y: 6.0, z: 9.0 }];
        let c = centroid(&pts).unwrap();
        assert!((c.x - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn centroid_empty_none() {
        assert!(centroid(&[]).is_none());
    }

    #[test]
    fn bounding_box_two_points() {
        let pts = vec![
            Coord3D { x: -1.0, y: -2.0, z: -3.0 },
            Coord3D { x:  1.0, y:  2.0, z:  3.0 },
        ];
        let (mn, mx) = bounding_box(&pts).unwrap();
        assert!((mn.x - -1.0).abs() < f64::EPSILON);
        assert!((mx.z -  3.0).abs() < f64::EPSILON);
    }
}
