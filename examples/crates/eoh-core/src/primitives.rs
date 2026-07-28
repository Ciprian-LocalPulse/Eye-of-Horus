//! Geometric primitives: vertices, edges, and shapes.

use crate::{
    coordinates::Coord3D,
    error::{EohError, EohResult},
    span::Span,
};
use serde::{Deserialize, Serialize};

/// A named point in 3-D space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    /// Programmer-visible identifier (e.g. `A`, `ORIGIN`).
    pub name: String,
    /// Spatial position.
    pub position: Coord3D,
    /// Source location.
    pub span: Span,
}

impl Vertex {
    /// Construct a vertex, validating the coordinate.
    pub fn new(name: impl Into<String>, x: f64, y: f64, z: f64, span: Span) -> EohResult<Self> {
        Ok(Self {
            name: name.into(),
            position: Coord3D::new(x, y, z)?,
            span,
        })
    }
}

/// A directed edge between two named vertices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// Source vertex name.
    pub from: String,
    /// Destination vertex name.
    pub to: String,
    /// Source location.
    pub span: Span,
}

/// The kind of geometric solid a [`Shape`] represents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeKind {
    /// A tetrahedron — four vertices, six edges, four triangular faces.
    Tetrahedron,
    /// An axis-aligned cube, defined by one anchor vertex and a side length.
    Cube,
    /// An icosahedron — twenty equilateral triangular faces, twelve vertices.
    Icosahedron,
    /// A user-defined polygon (arbitrary convex vertex list, ≥3 vertices).
    Polygon,
    /// A sphere defined by a centre vertex and a radius parameter.
    Sphere,
}

impl std::fmt::Display for ShapeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tetrahedron => write!(f, "TETRAHEDRON"),
            Self::Cube        => write!(f, "CUBE"),
            Self::Icosahedron => write!(f, "ICOSAHEDRON"),
            Self::Polygon     => write!(f, "POLYGON"),
            Self::Sphere      => write!(f, "SPHERE"),
        }
    }
}

/// A geometric solid constructed from named vertices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    /// Programmer-visible identifier.
    pub name: String,
    /// Solid variant.
    pub kind: ShapeKind,
    /// Names of the vertices that define this shape.
    pub vertices: Vec<String>,
    /// Optional scalar parameter (cube side length, sphere radius, …).
    pub param: Option<f64>,
    /// Source location.
    pub span: Span,
}

impl Shape {
    /// Validate vertex-count constraints for this [`ShapeKind`].
    pub fn validate_vertex_count(&self) -> EohResult<()> {
        let n = self.vertices.len();
        match &self.kind {
            ShapeKind::Tetrahedron if n != 4 => Err(EohError::Geometry(format!(
                "TETRAHEDRON requires exactly 4 vertices, got {n}"
            ))),
            ShapeKind::Icosahedron if n != 12 => Err(EohError::Geometry(format!(
                "ICOSAHEDRON requires exactly 12 vertices, got {n}"
            ))),
            ShapeKind::Polygon if n < 3 => Err(EohError::Geometry(format!(
                "POLYGON requires at least 3 vertices, got {n}"
            ))),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_span() -> Span { Span::new(0, 0, 0) }

    #[test]
    fn vertex_creation_ok() {
        let v = Vertex::new("A", 1.0, 2.0, 3.0, dummy_span());
        assert!(v.is_ok());
    }

    #[test]
    fn vertex_creation_nan_fails() {
        let v = Vertex::new("BAD", f64::NAN, 0.0, 0.0, dummy_span());
        assert!(v.is_err());
    }

    #[test]
    fn tetrahedron_vertex_count_validated() {
        let shape = Shape {
            name: "T".into(),
            kind: ShapeKind::Tetrahedron,
            vertices: vec!["A".into(), "B".into(), "C".into()], // only 3
            param: None,
            span: dummy_span(),
        };
        assert!(shape.validate_vertex_count().is_err());
    }
}
