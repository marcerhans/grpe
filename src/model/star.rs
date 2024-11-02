use renderer::VectorRow;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    vertices.append(&mut vec![
        VectorRow::from([0.0, 0.0, 0.0]),
        VectorRow::from([1.0, 0.0, -1.0]),
        VectorRow::from([0.5, 0.0, 1.0]),
        VectorRow::from([1.5, 0.0, 2.0]),
        VectorRow::from([0.4, 0.0, 2.0]),
        VectorRow::from([0.0, 0.0, 4.0]),
        VectorRow::from([-0.4, 0.0, 2.0]),
        VectorRow::from([-1.5, 0.0, 2.0]),
        VectorRow::from([-0.5, 0.0, 1.0]),
        VectorRow::from([-1.0, 0.0, -1.0]),
    ]);

    for vertex in vertices.iter_mut() {
        vertex.0.scale(32.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];

    // Faces
    lines.push(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0]); // Front

    lines
}
