/// Somewhat cool "spiral" when zooming and mutating FOV.

use std::{env, time::{self, Duration}};

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
    let mut vertices = vec![];

    // Spiral zooming in.
    const MAX_DEPTH: i32 = 1000;
    for i in 0..MAX_DEPTH {
        vertices.push(VectorRow::from([
            i as f64 * (((i as f64) / 16.0) % (std::f64::consts::PI * 2.0)).cos(),
            i as f64,
            i as f64 * (((i as f64) / 16.0) % (std::f64::consts::PI * 2.0)).sin(),
        ]));
    }

    const GRID_SIZE: i32 = 200;
    const GRID_SPACING: i32 = 100;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            vertices.push(VectorRow::from([
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
            rotation_quaternion: VectorRow::from([0.0, 0.0, 0.0, 0.0]),
            fov: 135,
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
    let fov_y_pos = (resolution.0 as f64 / 2.0) / f64::tan((renderer.config().camera.fov as f64 / 2.0) * (std::f64::consts::PI / 180.0));

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
        config.camera.position[1] = (config.camera.position[1] + 0.5) % 1000.0;

        let cam_pos_y_rotation = config.camera.position[1] - fov_y_pos / 2.0;
        config.camera.position[0] = cam_pos_y_rotation * ((cam_pos_y_rotation / 16.0) % (std::f64::consts::PI * 2.0)).cos();
        config.camera.position[2] = cam_pos_y_rotation * ((cam_pos_y_rotation / 16.0) % (std::f64::consts::PI * 2.0)).sin();
        angle = (angle + std::f64::consts::PI / 32.0) % (std::f64::consts::PI * 2.0);

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
            println!("Frame: {frame} | Missed Frames: {frame_missed} | FPS: {fps}");
            println!(
                "Resolution: ({},{}) | FOV: {:0>3}\nCamera Position: ({:.2},{:.2},{:.2})\nCamera Rotation: ({:.2},{:.2},{:.2},{:.2})",
                config.camera.resolution.0, config.camera.resolution.1, config.camera.fov,
                config.camera.position[0], config.camera.position[1], config.camera.position[2],
                config.camera.rotation_quaternion[0], config.camera.rotation_quaternion[1], config.camera.rotation_quaternion[2], config.camera.rotation_quaternion[3],
            );
        }
    }
}