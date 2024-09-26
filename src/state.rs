use std::{cell::RefCell, rc::Rc};

use io::{platform::unix::EventHandler, Event, EventHandlerTrait};
use linear_algebra::quaternion::{self, Quaternion};
use renderer::{Camera, RendererConfiguration, VectorRow};

mod input {
    pub mod mouse {
        pub enum Event {
            Down(f64, f64),
            Hold { from: (f64, f64), to: (f64, f64) },
            Up(f64, f64),
        }

        #[derive(Default)]
        pub struct State {
            pub left: Option<Event>,
            pub right: Option<Event>,
            pub scroll: Option<i32>,
        }
    }

    pub mod keyboard {
        pub enum Event {
            Down,
            Up,
        }

        #[derive(Default)]
        pub struct State {
            pub w: Option<Event>,
        }
    }

    #[derive(Default)]
    pub struct State {
        pub mouse: mouse::State,
        pub keyboard: keyboard::State,
    }
}

mod position {
    use renderer::VectorRow;

    pub struct State {
        pub value: VectorRow<f64, 3>,
    }

    impl Default for State {
        fn default() -> Self {
            Self {
                value: VectorRow::from([0.0, 0.0, 0.0]),
            }
        }
    }
}

mod rotation {
    pub struct State {
        pub value: (f64, f64),
    }

    impl Default for State {
        fn default() -> Self {
            Self { value: (0.0, 0.0) }
        }
    }
}

pub struct State {
    pub event_handler: EventHandler,
    pub vertices: Rc<RefCell<Vec<VectorRow<f64, 3>>>>,
    pub line_draw_order: Rc<RefCell<Vec<Vec<usize>>>>,
    input: input::State,
    position: position::State,
    rotation: rotation::State,
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
            input: Default::default(),
            position: Default::default(),
            rotation: Default::default(),
        }
    }

    pub fn update(&mut self, mut config: RendererConfiguration) -> RendererConfiguration {
        while let Ok(event) = self.event_handler.latest_event() {
            // Batch handling - Read all inputs up until this point.
            self.handle_event(event);
        }
        self.position.value = config.camera.position.clone(); // Not needed, but want to keep "real" state in [State] (semantics).
        config.camera = self.update_camera(config.camera);
        config
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Mouse(_modifier, event) => match (_modifier, event) {
                (io::Modifier::None, io::mouse::Event::Left(motion, x, y)) => match motion {
                    io::mouse::Motion::Down => {
                        if let Some(event) = self.input.mouse.left.as_ref() {
                            match event {
                                input::mouse::Event::Down(x_, y_) => {
                                    self.input.mouse.left = Some(input::mouse::Event::Hold {
                                        from: (*x_, *y_),
                                        to: (x as f64, -(y as f64)),
                                    });
                                }
                                input::mouse::Event::Hold { from, to: _ } => {
                                    self.input.mouse.left = Some(input::mouse::Event::Hold {
                                        from: (from.0, from.1),
                                        to: (x as f64, -(y as f64)),
                                    });
                                }
                                input::mouse::Event::Up(_x, _y) => unreachable!(),
                            }
                        } else {
                            self.input.mouse.left =
                                Some(input::mouse::Event::Down(x as f64, -(y as f64)))
                        }
                    }
                    io::mouse::Motion::Up => self.input.mouse.left = None,
                },
                (io::Modifier::None, io::mouse::Event::Middle(motion, x, y)) => match motion {
                    io::mouse::Motion::Down => todo!(),
                    io::mouse::Motion::Up => (),
                },
                (io::Modifier::None, io::mouse::Event::Right(motion, x, y)) => match motion {
                    io::mouse::Motion::Down => {
                        if let Some(event) = self.input.mouse.right.as_ref() {
                            match event {
                                input::mouse::Event::Down(x_, y_) => {
                                    self.input.mouse.right = Some(input::mouse::Event::Hold {
                                        from: (*x_, *y_),
                                        to: (x as f64, -(y as f64)),
                                    });
                                }
                                input::mouse::Event::Hold { from, to: _ } => {
                                    self.input.mouse.right = Some(input::mouse::Event::Hold {
                                        from: (from.0, from.1),
                                        to: (x as f64, -(y as f64)),
                                    });
                                }
                                input::mouse::Event::Up(_x, _y) => unreachable!(),
                            }
                        } else {
                            self.input.mouse.right =
                                Some(input::mouse::Event::Down(x as f64, -(y as f64)))
                        }
                    }
                    io::mouse::Motion::Up => self.input.mouse.right = None,
                },
                (io::Modifier::None, io::mouse::Event::Scroll(direction)) => match direction {
                    io::mouse::Direction::Down => match self.input.mouse.scroll.as_mut() {
                        Some(val) => *val -= 10,
                        None => self.input.mouse.scroll = Some(-10),
                    }
                    io::mouse::Direction::Up => match self.input.mouse.scroll.as_mut() {
                        Some(val) => *val += 10,
                        None => self.input.mouse.scroll = Some(10),
                    }
                },
                _ => (),
            },
            Event::Character(c) => match c {
                'w' => self.input.keyboard.w = Some(input::keyboard::Event::Down),
                _ => (),
            },
        }
    }

    fn update_camera(&mut self, mut camera: Camera) -> Camera {
        // Calculate rotational change based on input.
        let mut rot_diff = (0.0, 0.0);
        if let Some(event) = self.input.mouse.left.as_ref() {
            if let input::mouse::Event::Hold { from, to } = event {
                rot_diff.0 = (to.0 - from.0);
                rot_diff.1 = (to.1 - from.1);
            }
        }
        if self.input.mouse.left.as_ref().is_some() {
            self.input.mouse.left = None;
        }
        if let Some(key) =self.input.keyboard.w.as_ref() {
            rot_diff.0 += 0.1;
            rot_diff.1 += 0.1;
        }
        if self.input.keyboard.w.as_ref().is_some() {
            self.input.keyboard.w = None;
        }
        self.rotation.value.0 += rot_diff.0;
        self.rotation.value.1 += rot_diff.1;

        // Calculate positional change based on input.
        let mut pos_diff = VectorRow::from([0.0, 0.0, 0.0]);
        if let Some(event) = self.input.mouse.right.as_ref() {
            if let input::mouse::Event::Hold { from, to } = event {
                pos_diff[0] = to.0 - from.0;
                pos_diff[2] = to.1 - from.1;
            }
        }
        if let Some(val) = self.input.mouse.scroll.as_mut() {
            pos_diff[1] += *val as f64;
        }
        self.input.mouse.scroll = None;
        self.position.value = (&self.position.value.0 + &pos_diff.0).into();

        // Apply updated rotation on positional change.
        self.rotation.value.0 = (self.rotation.value.0)
            .min(std::f64::consts::FRAC_PI_2)
            .max(-std::f64::consts::FRAC_PI_2);
        // self.rotation.value.1 += rot_diff.1;
        // let rotation = (self.rotation.value.0 / 2.0, self.rotation.value.1 / 2.0);
        // let pitch = Quaternion::new(
        //     rotation.0.cos(),
        //     // rotation.0.sin() * (rotation.1 * 2.0).cos(),
        //     0.0,
        //     rotation.0.sin(),// * (rotation.1 * 2.0).sin(),
        //     0.0,
        // );
        // let yaw = Quaternion::new(
        //     rotation.1.cos(),
        //     0.0,
        //     0.0,
        //     rotation.1.sin(),
        // );
        // let rotation = yaw;
        // let rotation_prim = rotation.inverse();
        // let pos_diff_rotated = quaternion::rotate(&pos_diff, &rotation, &rotation_prim);

        // Update camera
        println!("{:?}", self.rotation.value);
        camera.rotation = self.rotation.value;
        camera.position = self.position.value.clone();
        camera

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
    }
}
