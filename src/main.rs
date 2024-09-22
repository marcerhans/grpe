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

    // 1. Instantiate IO handler.
    let event_handler = EventHandler::init();

    // 2. Instantiate renderer.
    let camera_default = Camera {
        resolution: args.resolution.unwrap_or((64, 64)),
        position: VectorRow::from([0.0, 0.0, 0.0]),
        projection_mode: ProjectionMode::Perspective { fov: 90 },
        ..Default::default()
    };
    let mut renderer = TerminalBuilder::default()
        .with_camera(camera_default.clone())
        .expect("Bad camera config.")
        .build()
        .unwrap();

    // 3. Create vertices.
    let vertices = Rc::new(RefCell::new(
        args.model
            .as_ref()
            .unwrap_or(&model::Model::Plane)
            .get_vertices(),
    ));
    renderer.set_vertices(Rc::clone(&vertices));

    // 4. Define line order.
    let line_draw_order = Rc::new(RefCell::new(
        args.model
            .as_ref()
            .unwrap_or(&model::Model::Plane)
            .get_line_draw_order(),
    ));
    renderer.set_vertices_line_draw_order(Rc::clone(&line_draw_order));

    // 5. Create a state tracker.
    let mut state = State::new(
        event_handler,
        vertices,
        line_draw_order,
    );

    // 6. Engine loop
    renderer.clear_screen();
    while state.event_handler.running() {
        renderer.render();
        let updated_config = state.update(renderer.config().clone());
        renderer = renderer.set_config(updated_config).expect("Bad configuration.");

        // let banner_text = "GRPE";
        // let banner_fill_width = (camera_updated.resolution.0 as usize - banner_text.len()) / 2 - 1; // Note: "-1" for extra space(s).
        // let banner_char = "=";
        // let banner = banner_char.repeat(banner_fill_width);
        // print!("\x1B[H");
        // print!("\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner} {banner_text} {banner}\x1B[0m");
        // if camera_updated.resolution.0 % 2 != 0 {
        //     // Just make it nice even if odd.
        //     print!("\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner_char}\x1B[0m");
        // }
        // println!();
    }
}
