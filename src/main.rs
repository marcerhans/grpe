use std::{thread, time::{self, Duration}};

use linear_algebra::{matrix::Matrix, vector::VectorRow};
use renderer::{renderer::TerminalBuilder, Camera, RendererBuilderTrait, RendererTrait};

fn main() {
    // 1. Create vertices
    // let mut vertices = vec![
        // A
        // Matrix::from_array([[-16.0, 0.0, 0.0]]),
        // Matrix::from_array([[-16.0, 0.0, -16.0]]),
        // Matrix::from_array([[0.0, 0.0, -16.0]]),
        // Matrix::from_array([[15.0, 0.0, -16.0]]),
        // Matrix::from_array([[15.0, 0.0, 0.0]]),
        // Matrix::from_array([[15.0, 0.0, 15.0]]),
        // Matrix::from_array([[0.0, 0.0, 15.0]]),
        // Matrix::from_array([[-16.0, 0.0, 15.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // /A

        // B
        // Matrix::from_array([[0.0, 0.0, -3.0]]),
        // Matrix::from_array([[0.0, 0.0, -2.0]]),
        // Matrix::from_array([[0.0, 0.0, -1.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 1.0]]),
        // Matrix::from_array([[0.0, 0.0, 2.0]]),
        // Matrix::from_array([[0.0, 0.0, 3.0]]),
        // Matrix::from_array([[-1.0, 0.0, 3.0]]),
        // Matrix::from_array([[1.0, 0.0, 3.0]]),

        // Matrix::from_array([[-1.0, -4.0, -3.0]]),
        // Matrix::from_array([[1.0, -4.0, -3.0]]),
        // Matrix::from_array([[0.0, -4.0, -3.0]]),
        // Matrix::from_array([[0.0, -4.0, -2.0]]),
        // Matrix::from_array([[0.0, -4.0, -1.0]]),
        // Matrix::from_array([[0.0, -4.0, 0.0]]),
        // Matrix::from_array([[0.0, -4.0, 1.0]]),
        // Matrix::from_array([[0.0, -4.0, 2.0]]),
        // Matrix::from_array([[0.0, -4.0, 3.0]]),

        // Matrix::from_array([[0.0, -8.0, -3.0]]),
        // Matrix::from_array([[0.0, -8.0, -2.0]]),
        // Matrix::from_array([[0.0, -8.0, -1.0]]),
        // Matrix::from_array([[0.0, -8.0, 0.0]]),
        // Matrix::from_array([[0.0, -8.0, 1.0]]),
        // Matrix::from_array([[0.0, -8.0, 2.0]]),
        // Matrix::from_array([[0.0, -8.0, 3.0]]),
        // /B

        // C
        // Smiley
        // Matrix::from_array([[1.0, 0.0, -3.0]]),
        // Matrix::from_array([[1.0, 0.0, -2.0]]),
        // Matrix::from_array([[1.0, 0.0, -1.0]]),

        // Matrix::from_array([[-1.0, 0.0, -3.0]]),
        // Matrix::from_array([[-1.0, 0.0, -2.0]]),
        // Matrix::from_array([[-1.0, 0.0, -1.0]]),

        // Matrix::from_array([[-2.0, 0.0, 1.0]]),
        // Matrix::from_array([[-1.0, 0.0, 2.0]]),
        // Matrix::from_array([[0.0, 0.0, 2.0]]),
        // Matrix::from_array([[1.0, 0.0, 2.0]]),
        // Matrix::from_array([[2.0, 0.0, 1.0]]),

        // Circle

        // Canyon
        // Matrix::from_array([[-32.0, -10.0, -32.0]]),
        // Matrix::from_array([[-31.0, -10.0, -32.0]]),
        // Matrix::from_array([[-30.0, -10.0, -32.0]]),
        // Matrix::from_array([[-29.0, -10.0, -32.0]]),
        // Matrix::from_array([[-28.0, -10.0, -32.0]]),
        // Matrix::from_array([[-27.0, -10.0, -32.0]]),
        // Matrix::from_array([[-26.0, -10.0, -32.0]]),
        // Matrix::from_array([[-25.0, -10.0, -32.0]]),
        // Matrix::from_array([[-24.0, -10.0, -32.0]]),
        // Matrix::from_array([[-23.0, -10.0, -32.0]]),
        // Matrix::from_array([[-22.0, -10.0, -32.0]]),
        // Matrix::from_array([[-21.0, -10.0, -32.0]]),
        // Matrix::from_array([[-20.0, -10.0, -32.0]]),
        // Matrix::from_array([[-19.0, -10.0, -32.0]]),
        // Matrix::from_array([[-18.0, -10.0, -32.0]]),
        // Matrix::from_array([[-17.0, -10.0, -32.0]]),
        // Matrix::from_array([[-16.0, -10.0, -32.0]]),
        // Matrix::from_array([[-15.0, -10.0, -28.0]]),
        // Matrix::from_array([[-14.0, -10.0, -25.0]]),
        // Matrix::from_array([[-13.0, -10.0, -25.0]]),
        // Matrix::from_array([[-12.0, -10.0, -25.0]]),
        // Matrix::from_array([[-11.0, -10.0, -25.0]]),
        // Matrix::from_array([[-10.0, -10.0, -25.0]]),
        // Matrix::from_array([[-9.0, -10.0, -25.0]]),
        // Matrix::from_array([[-8.0, -10.0, -25.0]]),
        // Matrix::from_array([[-7.0, -10.0, -25.0]]),
        // Matrix::from_array([[-6.0, -10.0, -25.0]]),
        // Matrix::from_array([[-5.0, -10.0, -24.0]]),
        // Matrix::from_array([[-4.0, -10.0, -24.0]]),
        // Matrix::from_array([[-3.0, -10.0, -25.0]]),
        // Matrix::from_array([[-2.0, -10.0, -25.0]]),
        // /C

        // Matrix::from_array([[2.0, 0.0, 0.0]]),
        // Matrix::from_array([[4.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 2.0]]),
        // Matrix::from_array([[-2.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, -2.0]]),

        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),
        // Matrix::from_array([[0.0, 0.0, 0.0]]),

        // Matrix::from_array([[-7.0, 10.0, 0.0]]),
        // Matrix::from_array([[8.0, -10.0, 0.0]]),
    // ];

    // 2. Define line order
    let line_draw_order = vec![vec![0, 1], vec![0, 2]];

    // 3. Render()
    let mut renderer = TerminalBuilder::default()
        .with_camera(Camera {
            resolution: (32, 32),
            position: VectorRow::from([0.0, -1.0, 0.0]),
            fov: 90,
        })
        .build();

    // A
    // let mut angle = 0.0;

    let mut frame: u128 = 0;
    let mut frame_tmp: u128 = 0;
    let mut frame_timer = time::Instant::now();
    let mut fps = 0;

    loop {
        // Loop
        thread::sleep(Duration::from_millis(100));
        // renderer.set_vertices(&vertices);
        renderer.render();
        // *vertices[0].index_mut(0, 0) += 2.0;
        // // *vertices[0].index_mut(0, 1) += 5.0;
        // *vertices[0].index_mut(0, 2) += 1.0;

        // let mut config = renderer.config();
        // *config.camera.position.index_mut(0, 0) += 0.05;
        // *config.camera.position.index_mut(0, 1) += 0.05;
        // *config.camera.position.index_mut(0, 2) += 0.05;
        // let _ = renderer.set_config(config);

        // B C
        // let mut config = renderer.config();
        // *config.camera.position.index_mut(0, 0) += 0.05;
        // let _ = renderer.set_config(config);
        // /B /C

        // B C | 2
        // let mut config = renderer.config();
        // *config.camera.position.index_mut(0, 1) += 0.5;
        // let _ = renderer.set_config(config);
        // /B /C


        // A
        // let mut i: isize = 9;
        // while i > 0 {
        //     vertices[8+i as usize] = Matrix::from_array([[(5.0 + i as f64)*f64::cos(angle + angle * i as f64), 0.0, (6.0 + i as f64)*f64::sin(angle + angle * i as f64)]]);
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

        println!("Statistics: [FPS: {}]",  fps);
    }
}
