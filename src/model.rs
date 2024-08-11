use std::str::FromStr;

use renderer::VectorRow;

pub enum Model {
    Plane,
    Spiral,
}

impl FromStr for Model {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "plane" => Ok(Model::Plane),
            "spiral" => Ok(Model::Spiral),
            _ => Err("Could not convert to string."),
        }
    }
}

impl Model {
    pub fn get_vertices(&self) -> Vec<VectorRow<f64, 3>> {
        let mut vertices = Vec::new();

        match self {
            Model::Plane => {
                vertices = vec![
                    // Sidroder
                    VectorRow::from([0.6, 3.0, 0.0]),
                    VectorRow::from([1.0, 3.0, 0.0]),
                    VectorRow::from([1.4, 0.5, 0.0]),
                    // Fuselage
                    VectorRow::from([0.0, 0.0, 0.0]),  // Exhaust
                    VectorRow::from([15.2, 0.0, 0.0]), // Tip
                    VectorRow::from([15.7, 0.0, 0.0]), // Tip + pitot
                    VectorRow::from([15.2 / 2.0, -1.0, 0.0]), // Bottom at middle
                    // Left wing
                    VectorRow::from([1.2, 0.0, 4.3]), // Wing span is 8.6 (/ 2 = 4.3)
                    VectorRow::from([2.0, 0.0, 4.3]),
                    VectorRow::from([15.2 / 2.0, 0.0, 1.0]),
                    // Left canard wing
                    VectorRow::from([15.2 / 2.0, 0.0, 4.3 / 1.75]), // Wing span is 8.6 (/ 2 = 4.3)
                    VectorRow::from([15.2 / 2.0 + 0.4, 0.0, 4.3 / 1.75]),
                    VectorRow::from([15.2 / 2.0 + 2.0, 0.0, 1.0]),
                ];

                for vertex in vertices.iter_mut() {
                    vertex.0.scale(10.0);
                }
            }
            Model::Spiral => {
                // Spiral zooming in.
                const MAX_DEPTH: i32 = 1000;
                for i in 0..MAX_DEPTH {
                    vertices.push(VectorRow::from([
                        i as f64 * (((i as f64) / 16.0) % (std::f64::consts::PI * 2.0)).cos(),
                        i as f64,
                        i as f64 * (((i as f64) / 16.0) % (std::f64::consts::PI * 2.0)).sin(),
                    ]));
                }

                const GRID_SIZE: i32 = 200;
                const GRID_SPACING: i32 = 100;
                for i in 0..GRID_SIZE {
                    for j in 0..GRID_SIZE {
                        vertices.push(VectorRow::from([
                            (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (i * GRID_SPACING) as f64,
                            MAX_DEPTH as f64,
                            (-GRID_SIZE / 2 * GRID_SPACING) as f64 + (j * GRID_SPACING) as f64,
                        ]));
                    }
                }
            }
        }

        vertices
    }

    pub fn get_line_draw_order(&self) -> Vec<Vec<u64>> {
        todo!()
    }
}
