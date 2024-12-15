use crate::vector::Vector3;

#[derive(Debug, Clone)]
pub struct Body {
    pub position: Vector3,
    pub velocity: Vector3,
    pub mass: f64
}

impl Body {
    pub fn new(position: Vector3, velocity: Vector3, mass: f64) -> Self {
        Body {
            position,
            velocity,
            mass
        }
    }
}
