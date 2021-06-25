#[derive(Debug)]
/// Returns list of intersections, and the id of object that the ray intersected with
pub struct Intersection {
    pub list: Vec<f64>,
    pub object: u128,
}

impl Intersection {
    pub fn count(&self) -> usize {
        self.list.len()
    }
    pub fn get(&self, index: usize) -> Option<f64> {
        if index > self.list.len() {
            None
        } else {
            Some(self.list[index])
        }
    }
}
