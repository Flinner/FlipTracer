pub struct Intersection {
    pub list: Vec<f64>,
}

impl Intersection {
    pub fn count(&self) -> usize {
        self.list.len()
    }
}
