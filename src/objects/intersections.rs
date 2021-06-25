use super::sphere::Sphere;

#[derive(Debug, Clone, Copy)]
/// Returns list of intersections, and the id of object that the ray intersected with
pub struct Intersection {
    pub intersects_at: f64,
    pub object: Sphere,
}

#[derive(Debug)]
/// Returns list of intersections, and the id of object that the ray intersected with
pub struct Intersections {
    list: Vec<Intersection>,
}

impl Intersections {
    /// count of intersections, eq to `self.list.len()`
    pub fn count(&self) -> usize {
        self.list.len()
    }
    /// Returns Intersection
    pub fn get(&self, index: usize) -> Option<Intersection> {
        if index > self.list.len() {
            None
        } else {
            let intersects_at = self.list[index].intersects_at;
            let object = self.list[index].object;
            let intersection = Intersection {
                intersects_at,
                object,
            };
            Some(intersection)
        }
    }
    /// Returns `intersection_at: f64`
    pub fn get_intersection(&self, index: usize) -> Option<f64> {
        match self.get(index) {
            None => None,
            Some(intersection) => Some(intersection.intersects_at),
        }
    }

    /// Returns `intersection_at: f64`
    pub fn get_object(&self, index: usize) -> Option<Sphere> {
        match self.get(index) {
            None => None,
            Some(intersection) => Some(intersection.object),
        }
    }
}

impl Intersection {
    pub fn new(intersects_at: f64, object: Sphere) -> Self {
        Intersection {
            intersects_at,
            object,
        }
    }

    /// Returns `Intersections`
    pub fn agregate(&self, rhs: Self) -> Intersections {
        Intersections {
            list: vec![*self, rhs],
        }
    }
}
