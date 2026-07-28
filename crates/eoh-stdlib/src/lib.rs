//! # eoh-stdlib
//!
//! The Eye of Horus standard library.
//!
//! Provides built-in geometric operations, mathematical utilities, and spatial
//! combinators that are available without an explicit IMPORT in every `.eoh`
//! program.

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod geometry;
pub mod math;
pub mod spatial;

pub use geometry::{centroid, bounding_box, surface_area_tetra};
pub use math::{lerp, clamp, golden_angle};
pub use spatial::{nearest_vertex, vertices_within_radius};
