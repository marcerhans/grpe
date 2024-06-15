use std::{thread, time::Duration};

use linear_algebra::matrix::Matrix;
use renderer::{renderer::TerminalBuilder, Camera, RendererBuilderTrait, RendererTrait};

fn main() {
    // 1. Create vertices
    let mut vertices = [
        Matrix::from_array([[0.0, 0.0, -3.0]]),
        Matrix::from_array([[0.0, 0.0, -2.0]]),
        Matrix::from_array([[0.0, 0.0, -1.0]]),
        Matrix::from_array([[0.0, 0.0, 0.0]]),
        Matrix::from_array([[0.0, 0.0, 1.0]]),
        Matrix::from_array([[0.0, 0.0, 2.0]]),
        Matrix::from_array([[0.0, 0.0, 3.0]]),
    ];

    // 2. Define line order
    let line_draw_order = vec![vec![0, 1], vec![0, 2]];

    // 3. Render()
    let mut renderer = TerminalBuilder::default()
        .with_camera(Camera::new(
            (32, 32),
            &[0.0, 0.0, 0.0],
            &[0.0, 1.0, 0.0],
            90,
        ))
        .build();

    loop {
        thread::sleep(Duration::from_millis(500));
        renderer.set_vertices(&vertices);
        renderer.render();
        // *vertices[0].index_mut(0, 0) += 2.0;
        // // *vertices[0].index_mut(0, 1) += 5.0;
        // *vertices[0].index_mut(0, 2) += 1.0;

        let mut config = renderer.config();
        *config.camera.position.index_mut(0, 0) += 1.0;
        let _ = renderer.set_config(config);
    }
}
