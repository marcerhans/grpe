use renderer::VectorRow;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    vertices.push(VectorRow::from([-0.25, 0.0, -0.25]));
    vertices.push(VectorRow::from([0.25, 0.0, -0.25]));
    vertices.push(VectorRow::from([-0.25, 0.0, 0.25]));
    vertices.push(VectorRow::from([0.25, 0.0, 0.25]));

    vertices.push(VectorRow::from([-0.5, 0.0, -0.5]));
    vertices.push(VectorRow::from([0.5, 0.0, -0.5]));
    vertices.push(VectorRow::from([-0.5, 0.0, 0.5]));
    vertices.push(VectorRow::from([0.5, 0.0, 0.5]));

    vertices.push(VectorRow::from([-1.0, -1.0, -1.0]));
    vertices.push(VectorRow::from([1.0, -1.0, -1.0]));
    vertices.push(VectorRow::from([1.0, -1.0, 1.0]));
    vertices.push(VectorRow::from([-1.0, -1.0, 1.0]));

    vertices.push(VectorRow::from([-1.0, 1.0, -1.0]));
    vertices.push(VectorRow::from([1.0, 1.0, -1.0]));
    vertices.push(VectorRow::from([1.0, 1.0, 1.0]));
    vertices.push(VectorRow::from([-1.0, 1.0, 1.0]));

    vertices.push(VectorRow::from([-1.0, 4.0, -1.0]));
    vertices.push(VectorRow::from([1.0, 4.0, -1.0]));
    vertices.push(VectorRow::from([1.0, 4.0, 1.0]));
    vertices.push(VectorRow::from([-1.0, 4.0, 1.0]));

    vertices.push(VectorRow::from([-0.25, 0.0, -0.25 + 1.25]));
    vertices.push(VectorRow::from([0.25, 0.0, -0.25 + 1.25]));
    vertices.push(VectorRow::from([-0.25, 0.0, 0.25 + 1.25]));
    vertices.push(VectorRow::from([0.25, 0.0, 0.25 + 1.25]));

    for vertex in vertices.iter_mut() {
        vertex.0.scale(32.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];

    // Single Point Particles.
    lines.push(vec![0]);
    lines.push(vec![1]);
    lines.push(vec![2]);
    lines.push(vec![3]);

    // Line Particles.
    lines.push(vec![4,5]);
    lines.push(vec![6,7]);
    lines.push(vec![20,23]); // Cross
    lines.push(vec![21,22]); // Cross

    // Faces
    lines.push(vec![8, 9, 10, 11]); // Front
    lines.push(vec![12, 15, 14, 13]); // Back
    lines.push(vec![12, 8, 11, 15]); // Left
    lines.push(vec![9, 13, 14, 10]); // Right
    lines.push(vec![11, 10, 14, 15]); // Top
    lines.push(vec![8, 12, 13, 9]); // Bottom
    lines.push(vec![16, 17, 18, 19]); // Backpanel

    lines
}
