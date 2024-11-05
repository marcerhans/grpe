/// Modelled with image of brazillian plane. Top down image. FOV: 15.
use std::f64::consts;

use renderer::VectorRow;

const LENGTH: f64 = 15.2;
const WING_SPAN: f64 = 8.6;
const HEIGHT: f64 = 4.5;

mod wings {
    use super::*;

    pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
        let mut vertices = vec![];
        
        vertices.append(&mut vec![
            // Thick inner flap.
            VectorRow::from([2.68, 1.05, 0.0]), // 0
            VectorRow::from([3.07, 1.5, 0.0]),  // 1
            VectorRow::from([3.12, 2.75, 0.0]), // 2
            VectorRow::from([3.7, 2.75, -0.1]), // 3
            VectorRow::from([3.7, 2.75, 0.1]),  // 4
            VectorRow::from([3.8, 1.01, -0.1]), // 5
            VectorRow::from([3.8, 1.01, 0.1]),  // 6
        ]);

        // Duplicate and mirror.
        // vertices.append(&mut mirror_y(&vertices));

        vertices
    }

    pub fn get_line_draw_order(start: usize) -> Vec<Vec<usize>> {
        let mut line_draw_order = vec![];

        // Thick inner flap.
        line_draw_order.append(&mut vec![
            vec![start + 0, start + 6, start + 4, start + 2, start + 1],
            vec![start + 0, start + 1, start + 2, start + 3, start + 5],
            vec![start + 4, start + 6, start + 5, start + 3],
            vec![start + 0, start + 5, start + 6],
            vec![start + 2, start + 4, start + 3],
        ]);

        // // Right (Mirror)
        // line_draw_order = add_lines(start + 23, line_draw_order);

        line_draw_order
    }
}

pub fn get_vertices() -> Vec<VectorRow<f64, 3>> {
    let mut vertices = vec![];

    // Length for reference
    // vertices.append(&mut vec![
    //     VectorRow::from([0.0, 0.0, 0.0]),
    //     VectorRow::from([LENGTH, 0.0, 0.0]),
    //     VectorRow::from([4.0, WING_SPAN / 2.0, 0.0]),
    //     VectorRow::from([4.0, -WING_SPAN / 2.0, 0.0]),
    // ]);

    // vertices.append(&mut exhaust::get_vertices());
    // vertices.append(&mut fuselage::get_vertices());
    // vertices.append(&mut rudder::get_vertices());
    vertices.append(&mut wings::get_vertices());
    // vertices.append(&mut canards::get_vertices());
    // vertices.append(&mut intake::get_vertices());

    // vertices.append(&mut cockpit::get_vertices());

    // Backdrop
    // const GRID_SIZE: i32 = 200;
    // const GRID_SPACING: i32 = 10;
    // for i in 0..GRID_SIZE {
    //     for j in 0..GRID_SIZE {
    //         vertices.push(VectorRow::from([
    //             (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (i * GRID_SPACING) as f64,
    //             (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (j * GRID_SPACING) as f64,
    //             -10 as f64,
    //         ]));
    //     }
    // }

    // Scale and center
    for vertex in vertices.iter_mut() {
        vertex[0] = vertex[0] - LENGTH / 2.0; // Center plane
        vertex[1] -= 0.15;
        vertex.0.scale(32.6);
    }

    vertices
}

pub fn get_line_draw_order() -> Vec<Vec<usize>> {
    let mut line_draw_order = vec![];

    let mut index_start = 0;
    // line_draw_order.append(&mut vec![
    //     vec![0, 1],
    //     vec![2, 3],
    // ]);

    // let mut exhaust = exhaust::get_line_draw_order(index_start);
    // index_start += exhaust::get_vertices().len();
    // line_draw_order.append(&mut exhaust);

    // let mut fuselage = fuselage::get_line_draw_order(index_start);
    // index_start += fuselage::get_vertices().len();
    // line_draw_order.append(&mut fuselage);

    // let mut rudder = rudder::get_line_draw_order(index_start);
    // index_start += rudder::get_vertices().len();
    // line_draw_order.append(&mut rudder);

    let mut wings = wings::get_line_draw_order(index_start);
    index_start += wings::get_vertices().len();
    line_draw_order.append(&mut wings);

    // let mut canards = canards::get_line_draw_order(index_start);
    // index_start += canards::get_vertices().len();
    // line_draw_order.append(&mut canards);

    // let mut intake = intake::get_line_draw_order(index_start);
    // index_start += intake::get_vertices().len();
    // line_draw_order.append(&mut intake);

    // for i in index_start..(index_start + 200*200) {
    //     line_draw_order.push(vec![i]);
    // }

    line_draw_order
}
