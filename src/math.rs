//! This module defines types for math depending on features set

#[cfg(feature = "math_glam")]
pub use glam::Vec3;

#[cfg(feature = "math_glam")]
/// Vector 3
pub type BVHVector3 = glam::Vec3;

#[cfg(feature = "math_glam")]
/// Vector 3
pub type BVHPoint3 = glam::Vec3;

#[cfg(not(feature = "math_glam"))]
use nalgebra::{Point3, Vector3};

#[cfg(not(feature = "math_glam"))]
/// Vector 3
pub type BVHVector3 = Vector3<f32>;

#[cfg(not(feature = "math_glam"))]
/// Vector 3
pub type BVHPoint3 = Point3<f32>;

#[cfg(feature = "math_glam")]
/// Cross Vector 3
pub fn bvh_vec_cross(a: BVHVector3, b: BVHVector3) -> BVHVector3 {
    a.cross(b)
}

#[cfg(not(feature = "math_glam"))]
/// Cross Vector 3
pub fn bvh_vec_cross(a: BVHVector3, b: BVHVector3) -> BVHVector3 {
    a.cross(&b)
}

#[cfg(feature = "math_glam")]
/// Dot Vector 3
pub fn bvh_vec_dot(a: BVHVector3, b: BVHVector3) -> f32 {
    a.dot(b)
}

#[cfg(not(feature = "math_glam"))]
/// Dot Vector 3
pub fn bvh_vec_dot(a: BVHVector3, b: BVHVector3) -> f32 {
    a.dot(&b)
}