/// Somewhat cool "spiral" when zooming and mutating FOV.

use std::{env, time::{self, Duration}};

use io::{platform::unix::EventHandler, Event, EventHandlerTrait};
use linear_algebra::vector::VectorRow;
use renderer::{renderer::TerminalBuilder, Camera, RendererBuilderTrait, RendererTrait};

mod args_list {
    pub const RESOLUTION: (usize, usize) = (0, 1);
    pub const SHOW_INFO: usize = 2;
}

fn main() {
    // 0. Read args
    let args: Vec<String> = env::args().skip(1).collect();
    let resolution: (u64, u64) = (
        args.get(args_list::RESOLUTION.0).unwrap_or(&"32".to_string()).parse().unwrap(),
        args.get(args_list::RESOLUTION.1).unwrap_or(&"32".to_string()).parse().unwrap(),
    );
    let show_info: bool = args.get(args_list::SHOW_INFO).unwrap_or(&"true".to_string()).parse().unwrap();

    // 1. Create vertices.
    // Written in meters. Order of points is generally "from back of plane to front".
    let mut vertices = vec![
        // Sidroder
        VectorRow::from([0.6, 3.0, 0.0]),
        VectorRow::from([1.0, 3.0, 0.0]),
        VectorRow::from([1.4, 0.5, 0.0]),

        // Fuselage
        VectorRow::from([0.0, 0.0, 0.0]),   // Exhaust
        VectorRow::from([15.2, 0.0, 0.0]),  // Tip
        VectorRow::from([15.7, 0.0, 0.0]),  // Tip + pitot
        VectorRow::from([15.2 / 2.0, -1.0, 0.0]),  // Bottom at middle

        // Left wing
        VectorRow::from([1.2, 0.0, 4.3]),   // Wing span is 8.6 (/ 2 = 4.3)
        VectorRow::from([2.0, 0.0, 4.3]),
        VectorRow::from([15.2 / 2.0, 0.0, 1.0]),

        // Left canard wing
        VectorRow::from([15.2 / 2.0, 0.0, 4.3 / 1.75]),   // Wing span is 8.6 (/ 2 = 4.3)
        VectorRow::from([15.2 / 2.0 + 0.4, 0.0, 4.3 / 1.75]),
        VectorRow::from([15.2 / 2.0 + 2.0, 0.0, 1.0]),
    ];

    for vertex in vertices.iter_mut() {
        vertex.0.scale(10.0);
    }

    // 2. Define line order.
    // let line_draw_order = vec![vec![0, 1], vec![0, 2]];

    // 3. Instantiate renderer.
    let mut renderer = TerminalBuilder::default()
        .with_camera(Camera {
            resolution: (resolution.0, resolution.1),
            position: VectorRow::from([80.0, -1000.0, 0.0]),
            rotation: VectorRow::from([0.0, 0.0, 0.0]),
            fov: 1,
        })
        .build();

    let mut frame: u128 = 0;
    let mut frame_tmp: u128 = 0;
    let mut frame_missed: u128 = 0;
    let mut update_timer = time::Instant::now();
    let fps_target = 60;
    let mut fps = 0;
    let time_target = Duration::from_micros(1000000 / fps_target);
    let mut time_wait = time_target;

    let mut angle: f64 = 0.0;

    let event_handler = EventHandler::init();
    let mut mouse_x_start = 0.0;
    let mut mouse_y_start = 0.0;
    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;

    // 4. Render
    loop {
        let start = std::time::Instant::now();
        loop {
            // Dead simple spin sleep
            if std::time::Instant::now() - start > time_wait {
                break;
            }
        }
        let start = std::time::Instant::now();

        renderer.set_vertices(&vertices);
        renderer.render();

        let mut config = renderer.config();
        match event_handler.get_latest_event() {
            Some(Event::Mouse(mouse_event)) => match mouse_event {
                io::MouseEvent::LeftDown(x, y) => {
                    mouse_x_start = x as f64;
                    mouse_y_start = y as f64;
                },
                io::MouseEvent::LeftMove(x, y) => {
                    mouse_x = x as f64 - mouse_x_start;
                    mouse_y = y as f64 - mouse_y_start;
                },
                io::MouseEvent::LeftUp(_, _) => {
                    mouse_x = 0.0;
                    mouse_y = 0.0;
                },
                io::MouseEvent::RightDown(x, y) => {
                    mouse_x_start = x as f64;
                    mouse_y_start = y as f64;
                },
                io::MouseEvent::RightMove(x, y) => {
                    mouse_x = (x as f64 - mouse_x_start) * 4.0;
                    mouse_y = (y as f64 - mouse_y_start) * 4.0;
                },
                io::MouseEvent::RightUp(_, _) => {
                    mouse_x = 0.0;
                    mouse_y = 0.0;
                },
                _ => (),
            },
            Some(Event::Letter(c)) => {
                match c {
                    // Axis movement
                    'a' => config.camera.position[0] -= 2.0,
                    'd' => config.camera.position[0] += 2.0,
                    'j' => config.camera.position[1] -= 2.0,
                    'k' => config.camera.position[1] += 2.0,
                    'w' => config.camera.position[2] += 2.0,
                    's' => config.camera.position[2] -= 2.0,
                    'A' => config.camera.position[0] -= 8.0,
                    'D' => config.camera.position[0] += 8.0,
                    'J' => config.camera.position[1] -= 8.0,
                    'K' => config.camera.position[1] += 8.0,
                    'W' => config.camera.position[2] += 8.0,
                    'S' => config.camera.position[2] -= 8.0,

                    // FOV
                    'q' => config.camera.fov -= 1,
                    'e' => config.camera.fov += 1,
                    'Q' => config.camera.fov -= 2,
                    'E' => config.camera.fov += 2,

                    _ => (),
                }
            }
            None => (),
        }
        config.camera.position[0] += mouse_x;
        config.camera.position[2] -= mouse_y; // Terminal coordinates are upsidedown.
        let _ = renderer.set_config(config.clone());

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

        if show_info {
            println!("\x1B[2HFrame: {frame} | Missed Frames: {frame_missed} | FPS: {fps}");
            println!(
                "Resolution: ({},{}) | FOV: {:0>3}\nCamera Position: ({:.2},{:.2},{:.2})\nCamera Rotation: ({:.2},{:.2},{:.2},{:.2})",
                config.camera.resolution.0, config.camera.resolution.1, config.camera.fov,
                config.camera.position[0], config.camera.position[1], config.camera.position[2],
                config.camera.rotation_quaternion[0], config.camera.rotation_quaternion[1], config.camera.rotation_quaternion[2], config.camera.rotation_quaternion[3],
            );
        }
    }
}