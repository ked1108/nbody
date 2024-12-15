use crate::body::Body;
use crate::vector::Vector3;
use crate::constants::G;

#[derive(Debug, Clone)]
pub struct NbodySim {
    pub bodies: Vec<Body>
}

impl NbodySim {
    pub fn new(bodies: Vec<Body>) -> Self {
        NbodySim {bodies}
    }

    pub fn compute_accelerations(&self) -> Vec<Vector3> {
        let n = self.bodies.len();
        let mut accelerations = vec![Vector3::new(0.0, 0.0, 0.0); n];

        for i in 0..n {
            for j  in 0..n {
                
                if i == j {
                    continue;
                }

                let body = &self.bodies[j];
                let r = self.bodies[i].position.sub(body.position);
                let r_mag_sqr = r.mag_squared();
                let r_norm = r.norm();
                let a = r_norm.scale(G * self.bodies[i].mass / r_mag_sqr);

                accelerations[i] = accelerations[i].add(a);
            }
        }
        accelerations
    }
}
