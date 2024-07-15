use std::{thread, time::{self, Duration}};

use linear_algebra::vector::VectorRow;
use renderer::{renderer::TerminalBuilder, Camera, RendererBuilderTrait, RendererTrait};

fn main() {
    // 1. Create vertices.
    let mut vertices = vec![
        // VectorRow::from([0.0,  50.0, 0.0]),
        // VectorRow::from([0.0, 50.0, 4.0]),
        // VectorRow::from([0.0, 50.0, -4.0]),
        // VectorRow::from([4.0,  50.0, 0.0]),
        // VectorRow::from([-4.0,  50.0, 0.0]),

        // VectorRow::from([0.0,  200.0, 0.0]),
        // VectorRow::from([-4.0, 200.0, 4.0]),
        // VectorRow::from([-4.0, 200.0, -4.0]),
        // VectorRow::from([4.0,  200.0, 4.0]),
        // VectorRow::from([4.0,  200.0, -4.0]),

        // VectorRow::from([0.0,  4.0, 0.0]),
        // VectorRow::from([-1.0,  8.0, 0.0]),
        // VectorRow::from([-2.0,  16.0, 0.0]),
        // VectorRow::from([4.0,  4.0, 0.0]),
        // VectorRow::from([4.0,  4.0, 0.0]),
        // VectorRow::from([8.0,  8.0, 0.0]),
        // VectorRow::from([16.0, 16.0, 0.0]),
        // VectorRow::from([32.0, 32.0, 0.0]),
        // VectorRow::from([64.0, 64.0, 0.0]),

        // Smiley
        // VectorRow::from([-2.0,  16.0, 4.0]),
        // VectorRow::from([-2.0,  16.0, 3.0]),
        // VectorRow::from([-2.0,  16.0, 2.0]),
        // VectorRow::from([-2.0,  16.0, 1.0]),
        // VectorRow::from([-2.0,  16.0, 0.0]),
        // VectorRow::from([-2.0,  16.0, -1.0]),

        // VectorRow::from([2.0,  16.0, 4.0]),
        // VectorRow::from([2.0,  16.0, 3.0]),
        // VectorRow::from([2.0,  16.0, 2.0]),
        // VectorRow::from([2.0,  16.0, 1.0]),
        // VectorRow::from([2.0,  16.0, 0.0]),
        // VectorRow::from([2.0,  16.0, -1.0]),

        // VectorRow::from([-3.0,  16.0, -3.0]),
        // VectorRow::from([-3.0,  16.0, -4.0]),
        // VectorRow::from([-2.0,  16.0, -5.0]),
        // VectorRow::from([-1.0,  16.0, -5.0]),
        // VectorRow::from([0.0,  16.0, -5.0]),
        // VectorRow::from([1.0,  16.0, -5.0]),
        // VectorRow::from([2.0,  16.0, -5.0]),
        // VectorRow::from([3.0,  16.0, -4.0]),
        // VectorRow::from([3.0,  16.0, -3.0]),
    ];

    // Spiral if zooming in.
    for i in 0..1000 {
        vertices.push(VectorRow::from([
            (i as f64 / 1.5) * ((i as f64) % (std::f64::consts::PI * 2.0)).cos(), 
            i as f64,
            (i as f64 / 1.5) * ((i as f64) % (std::f64::consts::PI * 2.0)).sin(), 
        ]));
    }

    // 2. Define line order.
    // let line_draw_order = vec![vec![0, 1], vec![0, 2]];

    // 3. Instantiate renderer.
    let mut renderer = TerminalBuilder::default()
        .with_camera(Camera {
            resolution: (128, 128),
            position: VectorRow::from([0.0, 0.0, 0.0]),
            fov: 171,
        })
        .build();

    // let mut angle = 0.0;

    let mut frame: u128 = 0;
    let mut frame_tmp: u128 = 0;
    let mut frame_timer = time::Instant::now();
    let mut fps = 0;

    // 4. Render
    loop {
        // Loop
        // thread::sleep(Duration::from_millis(1));
        renderer.set_vertices(&vertices);
        renderer.render();
        // *vertices[0].index_mut(0, 0) += 2.0;
        // // *vertices[0].index_mut(0, 1) += 5.0;
        // *vertices[0].index_mut(0, 2) += 1.0;

        let mut config = renderer.config();
        // config.camera.position[0] += 1.0;
        config.camera.position[1] += 0.05;
        // config.camera.position[2] += 0.5;

        // config.camera.fov += 1;
        // if config.camera.fov == 100 {
        //     config.camera.fov = 1;
        // }
        let _ = renderer.set_config(config.clone());

        // A
        // let mut i: isize = 9;
        // while i > 0 {
        //     vertices[8+i as usize] = VectorRow::from([(5.0 + i as f64)*f64::cos(angle + angle * i as f64), 0.0, (6.0 + i as f64)*f64::sin(angle + angle * i as f64)]);
        //     i -= 1;
        // }
        // angle += 0.01;
        // if angle > 2.0 * std::f64::consts::PI {
        //     angle = 0.0;
        // }

        // Statistics
        if frame_timer.elapsed() >= Duration::from_secs(1) {
            fps = frame_tmp;
            frame_tmp = 0;
            frame_timer = time::Instant::now();
        } else {
            frame_tmp += 1;
        }

        frame += 1;

        println!("Statistics: [Frame: {frame} | FPS: {fps}]");
        println!("Config: [Camera Position: {:?}]", config.camera.position);
    }
}
