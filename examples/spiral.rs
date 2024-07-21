/// Somewhat cool "spiral" when zooming and mutating FOV.
use std::{
    cell::RefCell,
    env,
    rc::Rc,
    time::{self, Duration},
};

use io::{platform::unix::EventHandler, Event, EventHandlerTrait};
use linear_algebra::vector::VectorRow;
use renderer::{renderer::TerminalBuilder, Camera, RendererBuilderTrait, RendererTrait};

mod args_list {
    pub const RESOLUTION: (usize, usize) = (0, 1);
    pub const SHOW_INFO: usize = 2;
}

mod ansi {
    pub static CLEAR_SCREEN: &str = "\x1B[2J";
    pub static GO_TO_0_0: &str = "\x1B[H";
}

fn main() {
    // 0. Read args
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(first_arg) = args.get(0) {
        match first_arg.as_str() {
            "-h" | "--help" => {
                println!("TODO: Add help section.");
                return;
            }
            _ => (),
        }
    }

    let resolution: (u64, u64) = (
        args.get(args_list::RESOLUTION.0)
            .unwrap_or(&"32".to_string())
            .parse()
            .unwrap(),
        args.get(args_list::RESOLUTION.1)
            .unwrap_or(&"32".to_string())
            .parse()
            .unwrap(),
    );
    let show_info: bool = args
        .get(args_list::SHOW_INFO)
        .unwrap_or(&"true".to_string())
        .parse()
        .unwrap();

    // 1. Create vertices.
    let vertices = Rc::new(RefCell::new(vec![]));

    // Spiral zooming in.
    const MAX_DEPTH: i32 = 1000;
    for i in 0..MAX_DEPTH {
        vertices.borrow_mut().push(VectorRow::from([
            i as f64 * (((i as f64) / 16.0) % (std::f64::consts::PI * 2.0)).cos(),
            i as f64,
            i as f64 * (((i as f64) / 16.0) % (std::f64::consts::PI * 2.0)).sin(),
        ]));
    }

    const GRID_SIZE: i32 = 200;
    const GRID_SPACING: i32 = 100;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            vertices.borrow_mut().push(VectorRow::from([
                (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (i * GRID_SPACING) as f64,
                MAX_DEPTH as f64,
                (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (j * GRID_SPACING) as f64,
            ]));
        }
    }

    // 2. Define line order.
    // let line_draw_order = vec![vec![0, 1], vec![0, 2]];

    // 3. Instantiate renderer.
    let mut renderer = TerminalBuilder::default()
        .with_camera(Camera {
            resolution: (resolution.0, resolution.1),
            position: VectorRow::from([0.0, 0.0, 0.0]),
            rotation: VectorRow::from([0.0, 0.0, 0.0]),
            fov: 135,
        })
        .expect("Bad camera config.")
        .build()
        .unwrap();

    let mut frame: u128 = 0;
    let mut frame_tmp: u128 = 0;
    let mut frame_missed: u128 = 0;
    let mut update_timer = time::Instant::now();
    let fps_target = 60;
    let mut fps = 0;
    let time_target = Duration::from_micros(1000000 / fps_target);
    let mut time_wait = time_target;

    let event_handler = EventHandler::init();
    let mut mouse_x_start = 0.0;
    let mut mouse_y_start = 0.0;
    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;

    print!("{}", ansi::CLEAR_SCREEN); // TODO: Should be something like renderer.clear_screen().

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

        renderer.set_vertices(Rc::clone(&vertices));
        renderer.render();

        let mut camera = renderer.config().camera.clone();

        match event_handler.get_latest_event() {
            Some(Event::Mouse(mouse_event)) => match mouse_event {
                io::Mouse::LeftDown(x, y) => {
                    mouse_x_start = x as f64;
                    mouse_y_start = y as f64;
                },
                io::Mouse::LeftMove(x, y) => {
                    mouse_x = x as f64 - mouse_x_start;
                    mouse_y = y as f64 - mouse_y_start;
                },
                io::Mouse::LeftUp(_, _) => {
                    mouse_x = 0.0;
                    mouse_y = 0.0;
                },
                io::Mouse::RightDown(x, y) => {
                    mouse_x_start = x as f64;
                    mouse_y_start = y as f64;
                },
                io::Mouse::RightMove(x, y) => {
                    mouse_x = (x as f64 - mouse_x_start) * 4.0;
                    mouse_y = (y as f64 - mouse_y_start) * 4.0;
                },
                io::Mouse::RightUp(_, _) => {
                    mouse_x = 0.0;
                    mouse_y = 0.0;
                },
                _ => (),
            },
            Some(Event::Letter(c)) => {
                match c {
                    // Axis movement
                    'a' => camera.position[0] -= 2.0,
                    'd' => camera.position[0] += 2.0,
                    'j' => camera.position[1] -= 2.0,
                    'k' => camera.position[1] += 2.0,
                    'w' => camera.position[2] += 2.0,
                    's' => camera.position[2] -= 2.0,
                    'A' => camera.position[0] -= 8.0,
                    'D' => camera.position[0] += 8.0,
                    'J' => camera.position[1] -= 8.0,
                    'K' => camera.position[1] += 8.0,
                    'W' => camera.position[2] += 8.0,
                    'S' => camera.position[2] -= 8.0,

                    // FOV
                    'q' => camera.fov -= 1,
                    'e' => camera.fov += 1,
                    'Q' => camera.fov -= 2,
                    'E' => camera.fov += 2,

                    _ => (),
                }
            }
            None => (),
        }

        camera.position[0] += mouse_x;
        camera.position[2] -= mouse_y; // Terminal coordinates are upsidedown.

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

        if show_info {
            print!("\x1B[2KFrame: {frame} | Missed Frames: {frame_missed} | FPS: {fps} | Resolution: ({},{}) | FOV: {:0>3} | Camera Position: ({:.2},{:.2},{:.2}) | Camera Rotation: ({:.2},{:.2},{:.2})",
                camera.resolution.0, camera.resolution.1, camera.fov,
                camera.position[0], camera.position[1], camera.position[2],
                camera.rotation[0], camera.rotation[1], camera.rotation[2]
            );
        }

        let banner_text = "GRPE";
        let banner_fill_width = (camera.resolution.0 as usize - banner_text.len()) / 2 - 1; // Note: "-1" for extra space(s).
        let banner_char = "=";
        let banner = banner_char.repeat(banner_fill_width);
        print!("{}", ansi::GO_TO_0_0);
        print!("\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner} {banner_text} {banner}\x1B[0m");
        if camera.resolution.0 % 2 != 0 {
            // Just make it nice even if odd.
            print!("\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner_char}\x1B[0m");
        }
        println!();
    }
}
