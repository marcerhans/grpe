use std::f64::consts;

use renderer::VectorRow;

const LENGTH: f64 = 15.2;
const WING_SPAN: f64 = 8.6;
const HEIGHT: f64 = 4.5;
const UNIT: f64 = 15.2 / 21.0;

mod exhaust {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();
        let points = 24.0;
        let radius = UNIT / 2.25;

        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                0.0,
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                radius * (around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
            ]));
        }

        let radius = (UNIT / 2.0) * 1.5;
        for around_x_axis in 0..(points as usize) {
            vertices.push(VectorRow::from([
                UNIT,
                radius * (consts::PI / 10.0 + around_x_axis as f64 * (2.0 * consts::PI / points)).sin(),
                radius * (consts::PI / 10.0 + around_x_axis as f64 * (2.0 * consts::PI / points)).cos(),
            ]));
        }

        vertices
    }
}

mod fuselage {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        vertices.append(&mut vec![
            VectorRow::from([LENGTH, 0.0, 0.0]),
            VectorRow::from([LENGTH + 0.5, 0.0, 0.0]),
        ]);

        vertices
    }
}

mod rudder {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        vertices.append(&mut vec![
            VectorRow::from([1.1, 0.0, 0.65]),
            VectorRow::from([1.3, 0.0, 3.0 - UNIT]),
            VectorRow::from([1.3, 0.0, 3.0]),
            VectorRow::from([1.3 + UNIT, 0.0, 3.0]),
            VectorRow::from([1.3 + UNIT + UNIT * 4.0, 0.0, 1.0]),
        ]);

        vertices
    }
}

mod left_wing {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        vertices.append(&mut vec![
            VectorRow::from([4.0 * UNIT, 2.0 * UNIT, 0.0]),
            VectorRow::from([4.5 * UNIT, (2.0 + 0.5) * UNIT, 0.0]),
            VectorRow::from([4.5 * UNIT, (WING_SPAN / 2.0), 0.0]),
            VectorRow::from([(4.5 + 1.0) * UNIT, (WING_SPAN / 2.0), 0.0]),
            VectorRow::from([(6.5 + 7.0) * UNIT, 2.0 * UNIT, 0.0]),
        ]);

        vertices
    }
}

mod left_canard {
    use super::*;

    const RATIO_START: f64 = 370.0 / 650.0;
    const RATIO_LENGTH: f64 = 85.0 / 650.0;
    const REAL_LENGTH: f64 = RATIO_LENGTH * LENGTH;
    const START_X: f64 = RATIO_START * LENGTH;
    const END_X: f64 = START_X + REAL_LENGTH;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        vertices.append(&mut vec![
            VectorRow::from([START_X, 2.0 * UNIT, 0.25]),
            VectorRow::from([START_X, 2.2 * UNIT, 0.25]),
            VectorRow::from([START_X - 2.4 * UNIT, (WING_SPAN / 3.0), 0.25]),
            VectorRow::from([START_X, (WING_SPAN / 3.0), 0.25]),
            VectorRow::from([END_X, 2.0 * UNIT, 0.0]),
        ]);

        vertices
    }
}

mod cockpit {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        vertices.append(&mut vec![
            VectorRow::from([(6.5 + 7.0 + 0.5) * UNIT, 0.2, 1.0]),
            VectorRow::from([(6.5 + 7.0 + 0.5 * 5.0) * UNIT, 0.3, 1.0]),
            VectorRow::from([(6.5 + 7.0 + 0.5 * 5.0 + 0.5 * 7.5) * UNIT, 0.3, 1.0]),
            VectorRow::from([(6.5 + 7.0 + 0.5 * 5.0 + 0.5 * 7.5 + 0.5 * 4.0) * UNIT, 0.3, 1.0]),
        ]);

        let mut mirror = Vec::new();

        for vertex in &vertices {
            let mut v = vertex.clone();
            v[1] = -v[1];
            mirror.push(v);
        }

        vertices.append(&mut mirror);

        vertices
    }
}

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    vertices.append(&mut exhaust::get_vertices());
    vertices.append(&mut fuselage::get_vertices());
    vertices.append(&mut rudder::get_vertices());

    vertices.append(&mut left_wing::get_vertices());
    let mut right_wing = left_wing::get_vertices();
    for vertex in &mut right_wing {
        vertex[1] = -vertex[1];
    }
    vertices.append(&mut right_wing);

    vertices.append(&mut left_canard::get_vertices());
    let mut right_canard = left_canard::get_vertices();
    for vertex in &mut right_canard {
        vertex[1] = -vertex[1];
    }
    vertices.append(&mut right_canard);

    vertices.append(&mut cockpit::get_vertices());

    // Scale and center
    for vertex in vertices.iter_mut() {
        vertex[0] = vertex[0] - 15.7 / 2.0; // Center plane
        vertex.0.scale(16.0);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut lines = vec![];

    lines
}
