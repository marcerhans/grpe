use std::{cell::RefCell, io::Read, rc::Rc};

use io::{misc::CurrentSize, platform::unix::EventHandler, Event, EventHandlerTrait};
use linear_algebra::quaternion::{self, Quaternion};
use renderer::{Camera, ProjectionMode, RendererConfiguration, VectorRow};

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
        #![allow(non_snake_case)]
        /// This is ugly :)
        #[derive(Default)]
        pub struct State {
            pub r: Option<()>,
            pub o: Option<()>,
            pub w: Option<()>,
            pub v: Option<()>,
            pub i: Option<()>,
            pub j: Option<()>,
            pub k: Option<()>,
            pub l: Option<()>,
            pub f: Option<()>,
            pub F: Option<()>,
            pub plus: Option<()>,
            pub minus: Option<()>,
        }
    }

    pub mod misc {
        use std::{
            sync::{
                atomic::{self, AtomicBool, Ordering},
                Arc,
            },
            thread::{sleep, JoinHandle},
            time::Duration,
        };

        pub struct State {
            pub resize: Option<(u64, u64)>,
            pub resize_running: Arc<AtomicBool>,
            pub resize_querier: Option<JoinHandle<()>>,
        }

        impl Default for State {
            fn default() -> Self {
                let resize_running = Arc::new(AtomicBool::new(true));
                Self {
                    resize: Default::default(),
                    resize_querier: {
                        let running = Arc::clone(&resize_running);
                        Some(std::thread::spawn(move || {
                            while running.load(Ordering::Relaxed) {
                                // Query the terminal to give current dimensions.
                                // Response is handled by the event handler.
                                println!("\x1B[18t");
                                sleep(Duration::from_millis(500));
                            }
                        }))
                    },
                    resize_running,
                }
            }
        }

        impl Drop for State {
            fn drop(&mut self) {
                self.resize_running.store(false, Ordering::Relaxed);
                if let Some(handle) = self.resize_querier.take() {
                    handle.join().expect("Couldn't join on the associated thread.");
                }
            }
        }
    }

    #[derive(Default)]
    pub struct State {
        pub event_count: u64,
        pub mouse: mouse::State,
        pub keyboard: keyboard::State,
        pub misc: misc::State,
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

    pub fn update(&mut self, config: RendererConfiguration) -> RendererConfiguration {
        while let Ok(event) = self.event_handler.latest_event() {
            // Batch handling - Read all inputs up until this point.
            self.handle_event(event);
        }
        self.position.value = config.camera.position.clone(); // Not needed, but want to keep "real" state in [State] (semantics).
        self.update_config(config)
    }

    fn handle_event(&mut self, event: Event) {
        self.input.event_count += 1;

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
                        Some(val) => *val -= 1,
                        None => self.input.mouse.scroll = Some(-10),
                    },
                    io::mouse::Direction::Up => match self.input.mouse.scroll.as_mut() {
                        Some(val) => *val += 1,
                        None => self.input.mouse.scroll = Some(10),
                    },
                },
                _ => (),
            },
            Event::Character(c) => match c {
                'r' => self.input.keyboard.r = Some(()),
                'o' => self.input.keyboard.o = Some(()),
                'w' => self.input.keyboard.w = Some(()),
                'v' => self.input.keyboard.v = Some(()),
                'i' => self.input.keyboard.i = Some(()),
                'j' => self.input.keyboard.j = Some(()),
                'k' => self.input.keyboard.k = Some(()),
                'l' => self.input.keyboard.l = Some(()),
                'f' => self.input.keyboard.f = Some(()),
                'F' => self.input.keyboard.F = Some(()),
                '+' => self.input.keyboard.plus = Some(()),
                '-' => self.input.keyboard.minus = Some(()),
                _ => (),
            },
            Event::Misc(event) => match event {
                io::misc::Event::CurrentSize(current_size) => {
                    self.input.misc.resize = Some((current_size.0, current_size.1))
                }
            },
        }
    }

    fn update_config(&mut self, mut config: RendererConfiguration) -> RendererConfiguration {
        // Calculate rotational change based on input.
        let mut rot_diff = (0.0, 0.0);
        if let Some(event) = self.input.mouse.right.as_mut() {
            if let input::mouse::Event::Hold { from, to } = event {
                rot_diff.0 += (to.1 - from.1) * 0.02;
                rot_diff.1 += (to.0 - from.0) * -0.01;
                *event = input::mouse::Event::Down(to.0, to.1);
            }
        }

        if let Some(_) = self.input.keyboard.i.take() {
            rot_diff.0 -= std::f64::consts::FRAC_PI_4;
        }

        if let Some(_) = self.input.keyboard.k.take() {
            rot_diff.0 += std::f64::consts::FRAC_PI_4;
        }

        if let Some(_) = self.input.keyboard.j.take() {
            rot_diff.1 -= std::f64::consts::FRAC_PI_4;
        }

        if let Some(_) = self.input.keyboard.l.take() {
            rot_diff.1 += std::f64::consts::FRAC_PI_4;
        }

        // Calculate positional change based on input.
        let mut pos_diff = VectorRow::from([0.0, 0.0, 0.0]);
        if let Some(event) = self.input.mouse.left.as_mut() {
            if let input::mouse::Event::Hold { from, to } = event {
                pos_diff[0] = (from.0 - to.0) * 4.0;
                pos_diff[2] = (from.1 - to.1) * 8.0;
                *event = input::mouse::Event::Down(to.0, to.1);
            }
        }

        if let Some(val) = self.input.mouse.scroll.as_ref() {
            pos_diff[1] += (*val * 10) as f64;
            self.input.mouse.scroll = None;
        }

        if let Some(_) = self.input.keyboard.minus.take() {
            pos_diff[1] -= 10 as f64;
        }

        if let Some(_) = self.input.keyboard.plus.take() {
            pos_diff[1] += 10 as f64;
        }

        if let Some(_) = self.input.keyboard.v.take() {
            // Toggle view mode and adjust position
            if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
                // Undo any current rotation.
                let mut pos = quaternion::rotate(
                    &self.position.value,
                    &config.camera.rotation.1,
                    &config.camera.rotation.0,
                );

                config.camera.view_mode = match config.camera.view_mode {
                    renderer::ViewMode::FirstPerson => {
                        pos[1] += (config.camera.resolution.0 as f64 / 2.0)
                            / f64::tan((fov as f64 / 2.0) * (std::f64::consts::PI / 180.0));
                        renderer::ViewMode::Orbital
                    }
                    renderer::ViewMode::Orbital => {
                        pos[1] -= (config.camera.resolution.0 as f64 / 2.0)
                            / f64::tan((fov as f64 / 2.0) * (std::f64::consts::PI / 180.0));
                        renderer::ViewMode::FirstPerson
                    }
                };

                // Re-apply rotation.
                self.position.value =
                    quaternion::rotate(&pos, &config.camera.rotation.0, &config.camera.rotation.1);
            }
        }

        // Apply updated rotation on positional change.
        self.rotation.value.0 = (self.rotation.value.0 + rot_diff.0)
            .min(std::f64::consts::FRAC_PI_2)
            .max(-std::f64::consts::FRAC_PI_2);
        self.rotation.value.1 += rot_diff.1;

        let rotation = (self.rotation.value.0 / 2.0, self.rotation.value.1 / 2.0); // Half angles for quaternions.

        let pitch = Quaternion(
            rotation.0.cos(),
            rotation.0.sin() * (rotation.1 * 2.0).cos(),
            rotation.0.sin() * (rotation.1 * 2.0).sin(),
            0.0,
        );
        let yaw = Quaternion(rotation.1.cos(), 0.0, 0.0, rotation.1.sin());
        let rotation = &pitch * &yaw;
        let rotation_prim = rotation.inverse();
        pos_diff = quaternion::rotate(&pos_diff, &rotation, &rotation_prim);

        self.position.value = (&self.position.value.0 + &pos_diff.0).into();

        // Handle keyboard input
        if let Some(_) = self.input.keyboard.r.take() {
            // Reset
            self.rotation = Default::default();
            self.position = Default::default();
        }

        if let Some(_) = self.input.keyboard.o.take() {
            // Toggle render option
            config.option = match config.option {
                renderer::RenderOption::All => renderer::RenderOption::Line,
                renderer::RenderOption::Line => renderer::RenderOption::Vertices,
                renderer::RenderOption::Vertices => renderer::RenderOption::All,
            };
        }

        if let Some(_) = self.input.keyboard.F.take() {
            if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
                // Increase fov
                config.camera.projection_mode = ProjectionMode::Perspective { fov: fov + 5 };
            }
        }

        if let Some(_) = self.input.keyboard.f.take() {
            if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
                // Decrease fov
                config.camera.projection_mode = ProjectionMode::Perspective { fov: fov - 5 };
            }
        }

        // Resize
        if let Some(new_size) = self.input.misc.resize {
            config.camera.resolution.0 = new_size.0;
            config.camera.resolution.1 = new_size.1 * 2 - 5;
        }

        // Update camera
        config.camera.rotation = (rotation, rotation_prim);
        config.camera.position = self.position.value.clone();
        config
    }

    pub fn rotation(&self) -> (f64, f64) {
        self.rotation.value
    }

    pub fn event_count(&self) -> u64 {
        self.input.event_count
    }
}
