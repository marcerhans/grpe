use renderer::VectorRow;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    // Particles.
    vertices.push(VectorRow::from([-0.5, 0.0, -0.5]));
    vertices.push(VectorRow::from([0.5, 0.0, -0.5]));
    vertices.push(VectorRow::from([-0.5, 0.0, 0.5]));
    vertices.push(VectorRow::from([0.5, 0.0, 0.5]));

    // Vertices.
    vertices.push(VectorRow::from([-1.0, -1.0, -1.0]));
    vertices.push(VectorRow::from([1.0, -1.0, -1.0]));
    vertices.push(VectorRow::from([-1.0, -1.0, 1.0]));
    vertices.push(VectorRow::from([1.0, -1.0, 1.0]));

    vertices.push(VectorRow::from([-1.0, 1.0, -1.0]));
    vertices.push(VectorRow::from([1.0, 1.0, -1.0]));
    vertices.push(VectorRow::from([-1.0, 1.0, 1.0]));
    vertices.push(VectorRow::from([1.0, 1.0, 1.0]));

    for vertex in vertices.iter_mut() {
        vertex.0.scale(32.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];

    // Particles.
    lines.push(vec![0]);
    lines.push(vec![1]);
    lines.push(vec![2]);
    lines.push(vec![3]);

    // Lines.
    lines.push(vec![4,  5]);
    lines.push(vec![5,  7]);
    lines.push(vec![7,  6]);
    lines.push(vec![6,  4]);

    lines.push(vec![8,  9]);
    lines.push(vec![9,  11]);
    lines.push(vec![11, 10]);
    lines.push(vec![10,  8]);

    lines.push(vec![4,  8]);
    lines.push(vec![5,  9]);
    lines.push(vec![7,  11]);
    lines.push(vec![6,  10]);

    lines
}
