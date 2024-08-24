mod arg;
mod model;

use std::{
    cell::RefCell,
    rc::Rc,
    time::{self, Duration},
};

use io::{platform::unix::EventHandler, Event, EventHandlerTrait, MouseEvent};
use renderer::{
    renderer::TerminalBuilder, Camera, ProjectionMode, RendererBuilderTrait, RendererTrait, VectorRow, ViewMode
};

fn main() {
    let args = arg::parse_args();

    // 1. Create vertices.
    let vertices = Rc::new(RefCell::new(
        args.model.as_ref().unwrap_or(&model::Model::Plane).get_vertices(),
    ));

    // 2. Define line order.
    let line_draw_order = Rc::new(RefCell::new(args.model.as_ref().unwrap_or(&model::Model::Plane).get_line_draw_order()));

    // 3. Instantiate renderer.
    let camera_default = Camera {
        resolution: args.resolution.unwrap_or((64, 64)),
        position: VectorRow::from([0.0, 0.0, 0.0]),
        ..Default::default()
    };
    let mut renderer = TerminalBuilder::default()
        .with_camera(camera_default.clone())
        .expect("Bad camera config.")
        .build()
        .unwrap();

    let mut frame: u128 = 0;
    let mut frame_tmp: u128 = 0;
    let mut frame_missed: u128 = 0;
    let mut update_timer = time::Instant::now();
    let fps_target = args.fps.unwrap_or(60);
    let mut fps = 0;
    let time_target = Duration::from_micros(1000000 / fps_target);
    let mut time_wait = time_target;

    let event_handler = EventHandler::init();
    let mut mouse_left_x_start = 0.0;
    let mut mouse_left_y_start = 0.0;
    let mut mouse_left_x = 0.0;
    let mut mouse_left_y = 0.0;

    let mut mouse_right_x_start = 0.0;
    let mut mouse_right_y_start = 0.0;
    let mut mouse_right_x = 0.0;
    let mut mouse_right_y = 0.0;

    let mut camera_fov = if let ProjectionMode::Perspective { fov } = renderer.config().camera.projection_mode {
        fov
    } else {
        90
    };

    let show_info = args.info.is_some();
    let mut reset = false;

    renderer.clear_screen();

    // 4. Engine loop
    loop {
        let start = std::time::Instant::now();
        loop {
            if std::time::Instant::now() - start > time_wait {
                break;
            }
        }
        let start = std::time::Instant::now();

        renderer.set_vertices(Rc::clone(&vertices));
        renderer.set_vertices_line_draw_order(Rc::clone(&line_draw_order));
        renderer.render();

        let mut camera = renderer.config().camera.clone();

        match event_handler.get_latest_event() {
            Some(Event::Mouse(mouse_event)) => match mouse_event {
                MouseEvent::LeftDown(x, y) => {
                    mouse_left_x_start = x as f64;
                    mouse_left_y_start = y as f64;
                }
                MouseEvent::LeftMove(x, y) => {
                    mouse_left_x = x as f64 - mouse_left_x_start;
                    mouse_left_y = y as f64 - mouse_left_y_start;
                }
                MouseEvent::LeftUp(_, _) => {
                    mouse_left_x = 0.0;
                    mouse_left_y = 0.0;
                }
                MouseEvent::RightDown(x, y) => {
                    mouse_right_x_start = x as f64;
                    mouse_right_y_start = y as f64;
                }
                MouseEvent::RightMove(x, y) => {
                    mouse_right_x += (x as f64 - mouse_right_x_start) * 0.08;
                    mouse_right_y += (y as f64 - mouse_right_y_start) * 0.08;
                    mouse_right_x_start = x as f64;
                    mouse_right_y_start = y as f64;
                }
                MouseEvent::RightUp(_, _) => {
                    // mouse_right_x = 0.0;
                    // mouse_right_y = 0.0;
                }
                _ => (),
            },
            Some(Event::Letter(c)) => {
                match c {
                    // Axis movement
                    'a' => camera.position[0] -= 2.0,
                    'd' => camera.position[0] += 2.0,
                    // 'j' => camera.position[1] -= 2.0,
                    // 'k' => camera.position[1] += 2.0,
                    'w' => camera.position[2] += 2.0,
                    's' => camera.position[2] -= 2.0,
                    'A' => camera.position[0] -= 8.0,
                    'D' => camera.position[0] += 8.0,
                    // 'J' => camera.position[1] -= 8.0,
                    // 'K' => camera.position[1] += 8.0,
                    'W' => camera.position[2] += 8.0,
                    'S' => camera.position[2] -= 8.0,
                    '+' => camera.position[1] += 8.0,
                    '?' => camera.position[1] += 16.0,
                    '-' => camera.position[1] -= 8.0,
                    '_' => camera.position[1] -= 16.0,

                    // Rotation
                    'i' => camera.rotation.0 -= std::f64::consts::FRAC_PI_8,
                    'j' => camera.rotation.1 -= std::f64::consts::FRAC_PI_8,
                    'k' => camera.rotation.0 += std::f64::consts::FRAC_PI_8,
                    'l' => camera.rotation.1 += std::f64::consts::FRAC_PI_8,

                    // FOV
                    'q' | 'Q' | 'e' | 'E' => {
                        if let ProjectionMode::Perspective { fov } = &mut camera.projection_mode {
                            match c {
                                'q' => *fov -= 1,
                                'e' => *fov += 1,
                                'Q' => *fov -= 2,
                                'E' => *fov += 2,
                                _ => (),
                            }
                        }
                    }

                    // Projection Mode
                    'p' => {
                        match camera.projection_mode {
                            ProjectionMode::Orthographic => camera.projection_mode = ProjectionMode::Perspective { fov: camera_fov },
                            ProjectionMode::Perspective { fov } => {
                                camera_fov = fov;
                                camera.projection_mode = ProjectionMode::Orthographic;
                            }
                        }
                    }

                    // View mode
                    'v' => {
                        match camera.view_mode {
                            ViewMode::FirstPerson => camera.view_mode = ViewMode::Orbital,
                            ViewMode::Orbital => camera.view_mode = ViewMode::FirstPerson,
                        }
                    }

                    // Utils
                    'R' => reset = true,

                    _ => (),
                }
            }
            None => (),
        }

        camera.position[0] += mouse_left_x;
        camera.position[2] -= mouse_left_y; // Terminal coordinates are upsidedown.

        // camera.rotation.0 = -mouse_right_y;
        // camera.rotation.1 = -mouse_right_x;

        if let ViewMode::FirstPerson = camera.view_mode {
            camera.rotation.0 = f64::max(
                -std::f64::consts::FRAC_PI_2,
                f64::min(std::f64::consts::FRAC_PI_2, camera.rotation.0),
            );
        }

        if reset {
            reset = false;
            mouse_left_x_start = 0.0;
            mouse_left_y_start = 0.0;
            mouse_left_x = 0.0;
            mouse_left_y = 0.0;
            mouse_right_x_start = 0.0;
            mouse_right_y_start = 0.0;
            mouse_right_x = 0.0;
            mouse_right_y = 0.0;
            camera = camera_default.clone();
        }

        renderer = renderer.set_camera(camera.clone()).unwrap();

        // Statistics
        if update_timer.elapsed() >= Duration::from_secs(1) {
            fps = frame_tmp;
            frame_tmp = 0;
            update_timer = time::Instant::now();
        } else {
            frame_tmp += 1;
        }

        if let Some(time) = time_target.checked_sub(std::time::Instant::now() - start) {
            time_wait = time;
        } else {
            time_wait = Duration::from_micros(0);
            frame_missed += 1;
        }

        frame += 1;

        if let ProjectionMode::Perspective { fov } = camera.projection_mode {
            if show_info {
                print!("\x1B[2KFrame: {frame} | Missed Frames: {frame_missed} | FPS: {fps} | Resolution: ({},{}) | FOV: {:0>3} | Camera Position: ({:.2},{:.2},{:.2}) | Camera Rotation: (Pitch: {:.2}, Yaw: {:.2})",
                    camera.resolution.0, camera.resolution.1, fov,
                    camera.position[0], camera.position[1], camera.position[2],
                    camera.rotation.0, camera.rotation.1
                );
            }
        }

        let banner_text = "GRPE";
        let banner_fill_width = (camera.resolution.0 as usize - banner_text.len()) / 2 - 1; // Note: "-1" for extra space(s).
        let banner_char = "=";
        let banner = banner_char.repeat(banner_fill_width);
        print!("\x1B[H");
        print!("\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner} {banner_text} {banner}\x1B[0m");
        if camera.resolution.0 % 2 != 0 {
            // Just make it nice even if odd.
            print!("\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner_char}\x1B[0m");
        }
        println!();
    }
}
