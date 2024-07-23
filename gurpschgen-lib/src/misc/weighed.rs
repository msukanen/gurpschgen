pub trait Weighed {
    fn weight(&self) -> Option<f64>;
}
