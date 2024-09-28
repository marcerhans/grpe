use renderer::VectorRow;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    vertices.push(VectorRow::from([-1.0, -1.0, -1.0]));
    vertices.push(VectorRow::from([1.0, -1.0, -1.0]));
    vertices.push(VectorRow::from([-1.0, -1.0, 1.0]));
    vertices.push(VectorRow::from([1.0, -1.0, 1.0]));

    vertices.push(VectorRow::from([-1.0, 1.0, -1.0]));
    vertices.push(VectorRow::from([1.0, 1.0, -1.0]));
    vertices.push(VectorRow::from([-1.0, 1.0, 1.0]));
    vertices.push(VectorRow::from([1.0, 1.0, 1.0]));

    for vertex in vertices.iter_mut() {
        vertex.0.scale(200.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];

    lines.push(vec![0, 1]);
    lines.push(vec![1, 3]);
    lines.push(vec![3, 2]);
    lines.push(vec![2, 0]);

    lines.push(vec![4, 5]);
    lines.push(vec![5, 7]);
    lines.push(vec![7, 6]);
    lines.push(vec![6, 4]);

    lines.push(vec![0, 4]);
    lines.push(vec![1, 5]);
    lines.push(vec![3, 7]);
    lines.push(vec![2, 6]);

    lines
}
