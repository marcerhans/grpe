mod arg;
mod model;
mod state;

use std::{cell::RefCell, rc::Rc};

use state::State;
use io::{platform::unix::EventHandler, EventHandlerTrait};
use renderer::{
    renderer::TerminalBuilder, Camera, ProjectionMode, RendererBuilderTrait, RendererTrait,
    VectorRow,
};

fn main() {
    let args = arg::parse_args();

    // 1. Create vertices.
    let vertices = Rc::new(RefCell::new(
        args.model
            .as_ref()
            .unwrap_or(&model::Model::Plane)
            .get_vertices(),
    ));

    // 2. Define line order.
    let line_draw_order = Rc::new(RefCell::new(
        args.model
            .as_ref()
            .unwrap_or(&model::Model::Plane)
            .get_line_draw_order(),
    ));

    // 3. Instantiate IO handler.
    let event_handler = EventHandler::init();

    // 4. Instantiate renderer.
    let camera_default = Camera {
        resolution: args.resolution.unwrap_or((64, 64)),
        // position: VectorRow::from([0.0, -1000.0, 0.0]),
        projection_mode: ProjectionMode::Perspective { fov: 90 },
        ..Default::default()
    };
    let mut renderer = TerminalBuilder::default()
        .with_camera(camera_default.clone())
        .expect("Bad camera config.")
        .build()
        .unwrap();

    // 5. Create a state tracker
    // It is a tad unclear what should/should not be part of this structure.
    // It is used mainly for tracking:
    // - Changes to input
    // - Statistics
    let mut state = State::new();

    // 6. Engine loop
    renderer.clear_screen();
    while event_handler.running() {
        renderer.set_vertices(Rc::clone(&vertices));
        renderer.set_vertices_line_draw_order(Rc::clone(&line_draw_order));
        renderer.render();
        // let updated_camera = state.update(&event_handler, renderer.config().camera.clone());
        // renderer = renderer.set_camera(updated_camera.clone()).unwrap();

        // let banner_text = "GRPE";
        // let banner_fill_width = (camera.resolution.0 as usize - banner_text.len()) / 2 - 1; // Note: "-1" for extra space(s).
        // let banner_char = "=";
        // let banner = banner_char.repeat(banner_fill_width);
        // print!("\x1B[H");
        // print!("\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner} {banner_text} {banner}\x1B[0m");
        // if camera.resolution.0 % 2 != 0 {
        //     // Just make it nice even if odd.
        //     print!("\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner_char}\x1B[0m");
        // }
        // println!();
    }
}
