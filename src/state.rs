use std::{cell::RefCell, rc::Rc};

use io::{mouse, platform::unix::EventHandler, Event, EventHandlerTrait};
use renderer::{renderer::Terminal, Camera, RendererConfiguration, RendererTrait, VectorRow};

pub struct State {
    pub event_handler: EventHandler,
    pub vertices: Rc<RefCell<Vec<VectorRow<f64, 3>>>>,
    pub line_draw_order: Rc<RefCell<Vec<Vec<usize>>>>,
}

impl State {
    pub fn new(
        event_handler: EventHandler,
        vertices: Rc<RefCell<Vec<VectorRow<f64, 3>>>>,
        line_draw_order: Rc<RefCell<Vec<Vec<usize>>>>,
    ) -> Self {
        Self {
            event_handler,
            vertices,
            line_draw_order,
        }
    }

    pub fn update(&mut self, mut config: RendererConfiguration) -> RendererConfiguration {
        self.handle_event(self.event_handler.latest_event().expect("Failed to handle event."));
        config.camera = self.update_camera(config.camera);
        config
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Mouse(_modifier, event) => match (_modifier, event) {
                (io::Modifier::None, mouse::Event::Left(motion, x, y)) => match motion {
                    mouse::Motion::Down => todo!(),
                    mouse::Motion::Up => todo!(),
                },
                (io::Modifier::None, mouse::Event::Middle(motion, x, y)) => match motion {
                    mouse::Motion::Down => todo!(),
                    mouse::Motion::Up => todo!(),
                },
                (io::Modifier::None, mouse::Event::Right(motion, x, y)) => match motion {
                    mouse::Motion::Down => todo!(),
                    mouse::Motion::Up => todo!(),
                },
                (io::Modifier::None, mouse::Event::Scroll(direction)) => match direction {
                    mouse::Direction::Down => todo!(),
                    mouse::Direction::Up => todo!(),
                },
                _ => (),
            },
            Event::Character(c) => match c {
                _ => (),
            },
        }
    }

    fn update_camera(&mut self, camera: Camera) -> Camera {
        // rotation_total = (
        //     rotation_total.0 + rotation_diff.0 - mouse_right_y,
        //     rotation_total.1 + rotation_diff.1 - mouse_right_x,
        // );
        // let rotation = (rotation_total.0 / 2.0, rotation_total.1 / 2.0);
        // let pitch = Quaternion {
        //     q0: rotation.0.cos(),
        //     q1: rotation.0.sin() * (rotation.1 * 2.0).cos(),
        //     q2: rotation.0.sin() * (rotation.1 * 2.0).sin(),
        //     q3: 0.0,
        // };
        // let yaw = Quaternion {
        //     q0: rotation.1.cos(),
        //     q1: 0.0,
        //     q2: 0.0,
        //     q3: rotation.1.sin(),
        // };
        // let rotation = &pitch * &yaw;
        // let rotation_inverse = Quaternion {
        //     q0: rotation.q0,
        //     q1: -rotation.q1,
        //     q2: -rotation.q2,
        //     q3: -rotation.q3,
        // };

        // let position_new = quaternion::rotate(&position_diff, &rotation, &rotation_inverse);

        // camera.rotation = rotation_total;
        // camera.position = (&camera.position.0 + &position_new.0).into();

        // // Statistics
        // if update_timer.elapsed() >= Duration::from_secs(1) {
        //     fps = frame_tmp;
        //     frame_tmp = 0;
        //     update_timer = time::Instant::now();
        // } else {
        //     frame_tmp += 1;
        // }

        // if let Some(time) = time_target.checked_sub(std::time::Instant::now() - start) {
        //     time_wait = time;
        // } else {
        //     time_wait = Duration::from_micros(0);
        //     frame_missed += 1;
        // }

        // frame += 1;

        // if let ProjectionMode::Perspective { fov } = camera.projection_mode {
        //     if show_info {
        //         print!("\x1B[2KFrame: {frame} | Missed Frames: {frame_missed} | FPS: {fps} | Resolution: ({},{}) | FOV: {:0>3} | Camera Position: ({:.2},{:.2},{:.2}) | Camera Rotation: (Pitch: {:.2}, Yaw: {:.2})",
        //             camera.resolution.0, camera.resolution.1, fov,
        //             camera.position[0], camera.position[1], camera.position[2],
        //             camera.rotation.0, camera.rotation.1
        //         );
        //     }
        // }
        todo!()
    }
}
