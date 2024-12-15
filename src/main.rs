mod vector;
mod body;
mod constants;
mod simulation;
mod integrator;
mod visualiser;

use crate::vector::Vector3;
use crate::body::Body;
use crate::simulation::NbodySim;
use crate::integrator::RK4Integrator;
use crate::visualiser::Visualizer;

fn main() {
    let bodies = vec![
        Body::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), 1.0),
        Body::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 0.001),
        Body::new(Vector3::new(1.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0), 0.001),
    ];

    let mut simulation = NbodySim::new(bodies);

    let dt = 0.01;
    let steps = 1000;

    let start = std::time::Instant::now();
    for _step in 0..steps {
        RK4Integrator::integrate(&mut simulation, dt);

        // println!("Step {}:", step);
        // for body in &simulation.bodies {
        //     println!(
        //         "Position: {:?}, Velocity: {:?}",
        //         body.position, body.velocity
        //     );
        // }
    }
    let duration = start.elapsed();
    println!("Time elapsed in running the simulation is: {:?}", duration);
}