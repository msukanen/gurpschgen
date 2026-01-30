//! Various approximations.

const F64_APPROX_EPSILON: f64 = 0.00001;
const F32_APPROX_EPSILON: f32 = 0.001;

/// A generic trait for approximations.
pub trait Approx<T> {
    /// Check whether `self` ≅ `other`.
    fn approx(&self, other: T) -> bool;
}

impl Approx<f64> for f64 {
    fn approx(&self, other: f64) -> bool {
        self - F64_APPROX_EPSILON <= other && other <= self + F64_APPROX_EPSILON
    }
}

impl Approx<f32> for f32 {
    fn approx(&self, other: f32) -> bool {
        self - F32_APPROX_EPSILON <= other && other <= self + F32_APPROX_EPSILON
    }
}
