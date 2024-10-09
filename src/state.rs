use std::{cell::RefCell, rc::Rc, time::Instant};

use io::{platform::unix::EventHandler, Event, EventHandlerTrait};
use linear_algebra::quaternion::{self, Quaternion};
use renderer::{Camera, ProjectionMode, RenderOption, RendererConfiguration, VectorRow, ViewMode};

use crate::arg::Args;

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
            pub a: Option<()>,
            pub r: Option<()>,
            pub o: Option<()>,
            pub O: Option<()>,
            pub w: Option<()>,
            pub v: Option<()>,
            pub i: Option<()>,
            pub j: Option<()>,
            pub k: Option<()>,
            pub l: Option<()>,
            pub f: Option<()>,
            pub F: Option<()>,
            pub c: Option<()>,
            pub plus: Option<()>,
            pub minus: Option<()>,
        }
    }

    pub mod misc {
        pub struct State {
            pub resize: Option<(u64, u64)>,
        }

        impl Default for State {
            fn default() -> Self {
                Self {
                    resize: Default::default(),
                }
            }
        }
    }

    pub mod auto {
        pub struct State {
            pub rot_diff: (f64, f64),
        }

        impl Default for State {
            fn default() -> Self {
                Self {
                    rot_diff: (0.0, 0.01),
                }
            }
        }
    }

    pub struct State {
        pub mouse: mouse::State,
        pub keyboard: keyboard::State,
        pub misc: misc::State,
        pub auto: Option<auto::State>,
    }

    impl Default for State {
        fn default() -> Self {
            Self {
                mouse: Default::default(),
                keyboard: Default::default(),
                misc: Default::default(),
                auto: Some(auto::State::default()),
                // auto: None, // MODEL
            }
        }
    }
}

mod info {
    use std::time::Instant;

    use renderer::{RenderOption, VectorRow, ViewMode};

    pub struct State {
        pub event_count: u64,
        pub position: VectorRow<f64, 3>,
        pub rotation: (f64, f64),
        pub invert_colors: bool,
        pub time_prev: Instant,
        pub fps: u64,
        pub fps_smoothened: u64,
        pub view_mode: ViewMode,
        pub render_option: RenderOption,
    }

    impl Default for State {
        fn default() -> Self {
            Self {
                event_count: 0,
                position: VectorRow::from([0.0, 0.0, 0.0]),
                rotation: Default::default(),
                // rotation: (-std::f64::consts::FRAC_PI_2, 0.0), // MODEL
                invert_colors: false,
                time_prev: Instant::now(),
                fps: 0,
                fps_smoothened: 0,
                view_mode: Default::default(),
                render_option: Default::default(),
            }
        }
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

/// The one thing that truly does a little bit too much.
pub struct StateHandler {
    pub args: Args,
    pub event_handler: EventHandler,
    pub vertices: Rc<RefCell<Vec<VectorRow<f64, 3>>>>,
    pub line_draw_order: Rc<RefCell<Vec<Vec<usize>>>>,
    input: input::State,
    info: info::State,
}

impl StateHandler {
    pub fn new(
        args: Args,
        event_handler: EventHandler,
        vertices: Rc<RefCell<Vec<VectorRow<f64, 3>>>>,
        line_draw_order: Rc<RefCell<Vec<Vec<usize>>>>,
    ) -> Self {
        Self {
            args,
            event_handler,
            vertices,
            line_draw_order,
            input: Default::default(),
            info: Default::default(),
        }
    }

    pub fn update(&mut self, config: RendererConfiguration) -> RendererConfiguration {
        loop {
            let now = Instant::now();
            self.info.fps = (1000_000_u128
                .checked_div(now.duration_since(self.info.time_prev).as_micros()))
            .unwrap_or(99999) as u64;

            let target_fps = match self.args.fps {
                Some(fps) => fps,
                None => self.info.fps,
            };

            if self.info.fps <= target_fps {
                self.info.time_prev = now;
                break;
            }

            if let Some(fps) = self.args.fps {
                if fps < 200 {
                    // Use a mix of spin lock and actual sleep. This, 4 ms, is enough for sub 240 fps.
                    std::thread::sleep(std::time::Duration::from_millis(4));
                }
            }
        }

        let smooth_factor = 0.002;
        self.info.fps_smoothened = (smooth_factor * self.info.fps as f64
            + (1.0 - smooth_factor) * self.info.fps_smoothened as f64)
            as u64;

        println!("\x1B[H\x1B[18t"); // Query terminal for size. (Move to first row before printing/receiving, because it will be cleared anyway.)

        while let Ok(event) = self.event_handler.latest_event() {
            // Batch handling - Read all inputs up until this point.
            self.handle_event(event);
        }
        let config = self.update_config(config);

        config
    }

    fn handle_event(&mut self, event: Event) {
        self.info.event_count += 1;

        if let Some(_) = self.input.auto.as_ref() {
            match event {
                Event::Misc(_) => (),
                Event::Character(c) => match c.to_ascii_lowercase() {
                    'o' | 'v' | 'f' | 'c' => (),
                    _ => self.input.auto = None,
                },
                _ => self.input.auto = None,
            }
        }

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
                'a' => self.input.keyboard.a = Some(()),
                'r' => self.input.keyboard.r = Some(()),
                'o' => self.input.keyboard.o = Some(()),
                'O' => self.input.keyboard.O = Some(()),
                'w' => self.input.keyboard.w = Some(()),
                'v' => self.input.keyboard.v = Some(()),
                'i' => self.input.keyboard.i = Some(()),
                'j' => self.input.keyboard.j = Some(()),
                'k' => self.input.keyboard.k = Some(()),
                'l' => self.input.keyboard.l = Some(()),
                'f' => self.input.keyboard.f = Some(()),
                'F' => self.input.keyboard.F = Some(()),
                'c' => self.input.keyboard.c = Some(()),
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
        if let Some(_) = self.input.keyboard.r.take() {
            // Reset
            self.info.rotation = Default::default();
            config.camera = Camera::default();
            if let Some(res) = self.args.resolution {
                config.camera.resolution = res;
            }
            config.option = RenderOption::default();
        }

        if let Some(_) = self.input.keyboard.o.take() {
            // Toggle render option
            config.option = match config.option {
                renderer::RenderOption::Vertices => renderer::RenderOption::WireFrame,
                renderer::RenderOption::WireFrame => renderer::RenderOption::WireFrameAndParticles,
                renderer::RenderOption::WireFrameAndParticles => renderer::RenderOption::Culling,
                renderer::RenderOption::Culling => renderer::RenderOption::CullingAndParticles,
                renderer::RenderOption::CullingAndParticles => renderer::RenderOption::PolyfillAndCulling,
                renderer::RenderOption::PolyfillAndCulling => renderer::RenderOption::PolyfillAndCullingAndParticles,
                renderer::RenderOption::PolyfillAndCullingAndParticles => renderer::RenderOption::Vertices,
            };
        }

        if let Some(_) = self.input.keyboard.O.take() {
            // Toggle render option
            config.option = match config.option {
                renderer::RenderOption::Vertices => renderer::RenderOption::PolyfillAndCullingAndParticles,
                renderer::RenderOption::WireFrame => renderer::RenderOption::Vertices,
                renderer::RenderOption::WireFrameAndParticles => renderer::RenderOption::WireFrame,
                renderer::RenderOption::Culling => renderer::RenderOption::WireFrameAndParticles,
                renderer::RenderOption::CullingAndParticles => renderer::RenderOption::Culling,
                renderer::RenderOption::PolyfillAndCulling => renderer::RenderOption::CullingAndParticles,
                renderer::RenderOption::PolyfillAndCullingAndParticles => renderer::RenderOption::PolyfillAndCulling,
            };
        }

        if let Some(_) = self.input.keyboard.F.take() {
            if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
                // Increase fov
                config.camera.projection_mode = ProjectionMode::Perspective {
                    fov: (fov + 5).min(170),
                };
            }
        }

        if let Some(_) = self.input.keyboard.f.take() {
            if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
                // Decrease fov
                config.camera.projection_mode = ProjectionMode::Perspective {
                    fov: fov.checked_sub(5).unwrap_or(1).max(5),
                };
            }
        }

        if let None = self.args.resolution {
            if let Some(new_size) = self.input.misc.resize {
                // Resize
                config.camera.resolution.0 = new_size.0;
                config.camera.resolution.1 = new_size.1 * 2 - 4; // "* 2 - 4" because we want to make space for title and info.
            }
        }

        // Calculate rotational change(s) based on input.
        let mut rot_diff = (0.0, 0.0);
        if let Some(event) = self.input.mouse.left.as_mut() {
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

        // Calculate positional change(s) based on input.
        let mut pos_diff = VectorRow::from([0.0, 0.0, 0.0]);
        if let Some(event) = self.input.mouse.right.as_mut() {
            if let ViewMode::FirstPerson = config.camera.view_mode {
                if let input::mouse::Event::Hold { from, to } = event {
                    pos_diff[0] = (from.0 - to.0) * 4.0;
                    pos_diff[2] = (from.1 - to.1) * 8.0;
                    *event = input::mouse::Event::Down(to.0, to.1);
                }
            }
        }

        if let Some(val) = self.input.mouse.scroll.take() {
            pos_diff[1] += (val * 10) as f64;
        }

        if let Some(_) = self.input.keyboard.minus.take() {
            pos_diff[1] -= 10 as f64;
        }

        if let Some(_) = self.input.keyboard.plus.take() {
            pos_diff[1] += 10 as f64;
        }

        if let Some(_) = self.input.keyboard.v.take() {
            // Toggle view mode and adjust position.
            if let ProjectionMode::Perspective { fov } = config.camera.projection_mode {
                // Undo any current rotation.
                let mut pos = quaternion::rotate(
                    &config.camera.position,
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
                config.camera.position =
                    quaternion::rotate(&pos, &config.camera.rotation.0, &config.camera.rotation.1);
            }
        }

        // Automatic mode?
        if let Some(()) = self.input.keyboard.a.take() {
            if let None = self.input.auto {
                self.input.auto = Some(input::auto::State::default());
            } else {
                self.input.auto = None;
            }
        }

        if let Some(auto) = self.input.auto.as_ref() {
            self.info.rotation.0 = -std::f64::consts::FRAC_PI_6;
            rot_diff = auto.rot_diff;
        }

        // Apply updated rotation on positional change.
        let old_rotation = config.camera.rotation.clone();

        if !(rot_diff.0 == 0.0 && rot_diff.1 == 0.0) {
            self.info.rotation = (
                (self.info.rotation.0 + rot_diff.0)
                    .min(std::f64::consts::FRAC_PI_2)
                    .max(-std::f64::consts::FRAC_PI_2),
                (self.info.rotation.1 + rot_diff.1) % (std::f64::consts::PI * 2.0),
            );

            let rotation = (self.info.rotation.0 / 2.0, self.info.rotation.1 / 2.0); // Half angles for quaternions.
            let pitch = Quaternion(
                rotation.0.cos(),
                rotation.0.sin() * (rotation.1 * 2.0).cos(),
                rotation.0.sin() * (rotation.1 * 2.0).sin(),
                0.0,
            );
            let yaw = Quaternion(rotation.1.cos(), 0.0, 0.0, rotation.1.sin());
            let rotation = &pitch * &yaw;
            let rotation_prim = rotation.inverse();
            config.camera.rotation = (rotation, rotation_prim);
        }

        match config.camera.view_mode {
            ViewMode::FirstPerson => {
                pos_diff = quaternion::rotate(
                    &pos_diff,
                    &config.camera.rotation.0,
                    &config.camera.rotation.1,
                );
                config.camera.position = (&config.camera.position.0 + &pos_diff.0).into();
            }
            ViewMode::Orbital => {
                let mut pos =
                    quaternion::rotate(&config.camera.position, &old_rotation.1, &old_rotation.0);
                pos = (&pos.0 + &pos_diff.0).into();
                config.camera.position =
                    quaternion::rotate(&pos, &config.camera.rotation.0, &config.camera.rotation.1)
            }
        }

        // Store info
        self.info.position = config.camera.position.clone();
        // self.info.rotation = Already done.
        self.info.render_option = config.option.clone();
        self.info.view_mode = config.camera.view_mode.clone();

        if let Some(_) = self.input.keyboard.c.take() {
            self.info.invert_colors = !self.info.invert_colors;
        }

        config
    }

    pub fn info(&self) -> &info::State {
        &self.info
    }
}
