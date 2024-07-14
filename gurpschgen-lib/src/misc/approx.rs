pub trait Approx<T> {
    /**
     Check whether `self` ≅ `other`.
     */
    fn approx(&self, other: T) -> bool;
}

impl Approx<f64> for f64 {
    fn approx(&self, other: f64) -> bool {
        self - 0.000001 <= other && other <= self + 0.000001
    }
}

impl Approx<f32> for f32 {
    fn approx(&self, other: f32) -> bool {
        self - 0.001 <= other && other <= self + 0.001
    }
}
