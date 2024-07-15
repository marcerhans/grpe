use std::time::{self, Duration};

use linear_algebra::vector::VectorRow;
use renderer::{renderer::TerminalBuilder, Camera, RendererBuilderTrait, RendererTrait};

fn main() {
    // 1. Create vertices.
    let mut vertices = vec![
        // VectorRow::from([0.0, 0.0, 0.0]),
    ];

    // Spiral zooming in.
    for i in 0..1000 {
        vertices.push(VectorRow::from([
            0.75 * (i as f64 / 1.5) * ((i as f64) % (std::f64::consts::PI * 2.0)).cos(),
            i as f64,
            0.75 * (i as f64 / 1.5) * ((i as f64) % (std::f64::consts::PI * 2.0)).sin(),
        ]));
    }

    // 2. Define line order.
    // let line_draw_order = vec![vec![0, 1], vec![0, 2]];

    // 3. Instantiate renderer.
    let mut renderer = TerminalBuilder::default()
        .with_camera(Camera {
            resolution: (274, 131),
            position: VectorRow::from([0.0, 0.0, 0.0]),
            rotation_quaternion: VectorRow::from([0.0, 0.0, 0.0, 0.0]),
            fov: 164,
        })
        .build();

    let mut frame: u128 = 0;
    let mut frame_tmp: u128 = 0;
    let mut frame_missed: u128 = 0;
    let mut update_timer = time::Instant::now();
    let fps_target = 120;
    let mut fps = 0;
    let time_target = Duration::from_micros(1000000 / fps_target);
    let mut time_wait = time_target;

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
        // config.camera.position[0] += 1.0;
        config.camera.position[1] += 0.05;
        // config.camera.position[2] += 0.5;

        if frame % 10 == 0 {
            config.camera.fov += 1;
        }

        if config.camera.fov == 170 {
            config.camera.fov = 1;
        }
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

        println!("Statistics: [Frame: {frame} | Missed Frames: {frame_missed} | FPS: {fps}]");
        println!(
            "Resolution: ({},{}) | FOV: {}\nCamera Position: ({:.2},{:.2},{:.2})\nCamera Rotation: ({:.2},{:.2},{:.2},{:.2})",
            config.camera.resolution.0, config.camera.resolution.1, config.camera.fov,
            config.camera.position[0], config.camera.position[1], config.camera.position[2],
            config.camera.rotation_quaternion[0], config.camera.rotation_quaternion[1], config.camera.rotation_quaternion[2], config.camera.rotation_quaternion[3],
        );
    }
}

#[allow(unused)]
fn smiley() -> Vec<VectorRow<f64, 3>> {
    vec![
        VectorRow::from([-2.0,  16.0, 4.0]),
        VectorRow::from([-2.0,  16.0, 3.0]),
        VectorRow::from([-2.0,  16.0, 2.0]),
        VectorRow::from([-2.0,  16.0, 1.0]),
        VectorRow::from([-2.0,  16.0, 0.0]),
        VectorRow::from([-2.0,  16.0, -1.0]),

        VectorRow::from([2.0,  16.0, 4.0]),
        VectorRow::from([2.0,  16.0, 3.0]),
        VectorRow::from([2.0,  16.0, 2.0]),
        VectorRow::from([2.0,  16.0, 1.0]),
        VectorRow::from([2.0,  16.0, 0.0]),
        VectorRow::from([2.0,  16.0, -1.0]),

        VectorRow::from([-3.0,  16.0, -3.0]),
        VectorRow::from([-3.0,  16.0, -4.0]),
        VectorRow::from([-2.0,  16.0, -5.0]),
        VectorRow::from([-1.0,  16.0, -5.0]),
        VectorRow::from([0.0,  16.0, -5.0]),
        VectorRow::from([1.0,  16.0, -5.0]),
        VectorRow::from([2.0,  16.0, -5.0]),
        VectorRow::from([3.0,  16.0, -4.0]),
        VectorRow::from([3.0,  16.0, -3.0]),
    ]
}