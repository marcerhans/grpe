use renderer::VectorRow;

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    // vertices.push(VectorRow::from([1.0, 0.0, 0.0]));
    // vertices.push(VectorRow::from([-1.0, 0.0, 0.0]));
    // vertices.push(VectorRow::from([1.0, 0.0, 0.0]));
    // vertices.push(VectorRow::from([0.0, 0.0, 1.0]));
    // vertices.push(VectorRow::from([0.0, 0.0, -1.0]));

    for vertex in vertices.iter_mut() {
        // vertex.0.scale(1.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];

    // lines.push(vec![0,1]);
    // lines.push(vec![0,2]);
    // lines.push(vec![0,3]);
    // lines.push(vec![0,4]);

    lines
}