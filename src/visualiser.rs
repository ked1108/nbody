use crate::simulation::NbodySim;
use crate::integrator::RK4Integrator;
use crate::vector::Vector3;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event_loop::EventLoop,
    window::Window,
};

pub struct Visualiser {
    pixels: Pixels,
    width: u32,
    height: u32,
}

impl Visualiser {
    pub fn new(simulation: &NbodySim) -> Self {
        let width = 800;
        let height = 800;

        let mut event_loop = EventLoop::new();
        let window = Window::new(&event_loop)
            .unwrap();
        let surface_texture = SurfaceTexture::new(width, height, &window);
        let pixels = Pixels::new(width, height, &surface_texture).unwrap();

        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn run(&mut self, simulation: &mut NbodySim, dt: f64) {
        let mut event_loop = EventLoop::new();
        let mut frame_count = 0;

        event_loop.run(move |event, _, control_flow| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => {
                    if let winit::event::WindowEvent::CloseRequested = event {
                        *control_flow = winit::event::ControlFlow::Exit;
                    }
                }
                winit::event::Event::MainEventsCleared => {
                    self.pixels.get_frame().fill(0);

                    if frame_count % 2 == 0 {
                        RK4Integrator::integrate(&mut simulation, dt);
                    }

                    self.draw_bodies(simulation);

                    self.pixels.render().unwrap();
                    frame_count += 1;
                }
                _ => {}
            }
        });
    }

    fn draw_bodies(&mut self, simulation: &NbodySim) {
        let frame = self.pixels.get_frame();

        let scale = 100.0; 
        let offset = Vector3::new(self.width as f64 / 2.0, self.height as f64 / 2.0, 0.0);

        for body in &simulation.bodies {
            let pos_2d = self.project_to_2d(&body.position, scale, &offset);

            let x = pos_2d.x as u32;
            let y = pos_2d.y as u32;

            if x < self.width && y < self.height {
                let pixel_index = (y * self.width + x) as usize;
                frame[pixel_index * 4 + 0] = 255;
                frame[pixel_index * 4 + 1] = 255; 
                frame[pixel_index * 4 + 2] = 255;
                frame[pixel_index * 4 + 3] = 255; 
            }
        }
    }

    fn project_to_2d(&self, position: &Vector3, scale: f64, offset: &Vector3) -> Vector3 {
        Vector3::new(
            position.x * scale + offset.x,
            position.y * scale + offset.y,
            0.0,
        )
    }
}
