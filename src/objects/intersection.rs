#[derive(Debug)]
pub struct Intersection {
    pub list: Vec<f64>,
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
