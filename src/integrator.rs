use crate::simulation::NbodySim;
use crate::vector::Vector3;

pub struct RK4Integrator;

impl RK4Integrator {
    pub fn integrate(sim: &mut NbodySim, dt: f64) {
        let n = sim.bodies.len();

        let mut k1_pos = vec![Vector3::new(0.0, 0.0, 0.0); n];
        let mut k1_vel = vec![Vector3::new(0.0, 0.0, 0.0); n];
        let mut k2_pos = vec![Vector3::new(0.0, 0.0, 0.0); n];
        let mut k2_vel = vec![Vector3::new(0.0, 0.0, 0.0); n];
        let mut k3_pos = vec![Vector3::new(0.0, 0.0, 0.0); n];
        let mut k3_vel = vec![Vector3::new(0.0, 0.0, 0.0); n];
        let mut k4_pos = vec![Vector3::new(0.0, 0.0, 0.0); n];
        let mut k4_vel = vec![Vector3::new(0.0, 0.0, 0.0); n];

        let acc = sim.compute_accelerations();
        for (i, body) in sim.bodies.iter().enumerate() {
            k1_pos[i] = body.velocity.clone();   
            k1_vel[i] = acc[i].clone();         
        }

        for (i, body) in sim.bodies.iter_mut().enumerate() {
            let temp_pos = body.position.add(k1_pos[i].scale(dt / 2.0));
            let temp_vel = body.velocity.add(k1_vel[i].scale(dt / 2.0));
            body.position = temp_pos;
            body.velocity = temp_vel;
        }
        let acc_k2 = sim.compute_accelerations();
        for (i, body) in sim.bodies.iter().enumerate() {
            k2_pos[i] = body.velocity.clone();      
            k2_vel[i] = acc_k2[i].clone();         
        }

        for (i, body) in sim.bodies.iter_mut().enumerate() {
            let temp_pos = body.position.add(k2_pos[i].scale(dt / 2.0));
            let temp_vel = body.velocity.add(k2_vel[i].scale(dt / 2.0));
            body.position = temp_pos;
            body.velocity = temp_vel;
        }
        let acc_k3 = sim.compute_accelerations();
        for (i, body) in sim.bodies.iter().enumerate() {
            k3_pos[i] = body.velocity.clone();      
            k3_vel[i] = acc_k3[i].clone();          
        }

        for (i, body) in sim.bodies.iter_mut().enumerate() {
            let temp_pos = body.position.add(k3_pos[i].scale(dt));
            let temp_vel = body.velocity.add(k3_vel[i].scale(dt));
            body.position = temp_pos;
            body.velocity = temp_vel;
        }
        let acc_k4 = sim.compute_accelerations();
        for (i, body) in sim.bodies.iter().enumerate() {
            k4_pos[i] = body.velocity.clone();      
            k4_vel[i] = acc_k4[i].clone();          
        }

        for (i, body) in sim.bodies.iter_mut().enumerate() {
            body.position = body
                .position
                .add(k1_pos[i].scale(dt / 6.0))
                .add(k2_pos[i].scale(dt / 3.0))
                .add(k3_pos[i].scale(dt / 3.0))
                .add(k4_pos[i].scale(dt / 6.0));

            body.velocity = body
                .velocity
                .add(k1_vel[i].scale(dt / 6.0))
                .add(k2_vel[i].scale(dt / 3.0))
                .add(k3_vel[i].scale(dt / 3.0))
                .add(k4_vel[i].scale(dt / 6.0));
        }
    }
}