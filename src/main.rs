mod arg;
mod model;
mod state;

use std::{
    cell::RefCell,
    io::{stdout, BufWriter, Write},
    rc::Rc,
};

use io::{platform::unix::EventHandler, EventHandlerTrait};
use renderer::{
    renderer::terminal::TerminalBuilder, Camera, ProjectionMode,
    RendererBuilderTrait, RendererTrait,
};
use state::StateHandler;

fn main() {
    let args = arg::parse_args();

    // 1. Instantiate IO handler.
    let event_handler = EventHandler::init().expect("Failed to initialize event handler.");

    // let rotation: (f64, f64) = (-std::f64::consts::FRAC_PI_4, 0.0);
    // let pitch = linear_algebra::quaternion::Quaternion(
    //     rotation.0.cos(),
    //     rotation.0.sin() * (rotation.1 * 2.0).cos(),
    //     rotation.0.sin() * (rotation.1 * 2.0).sin(),
    //     0.0,
    // );
    // let yaw = linear_algebra::quaternion::Quaternion(rotation.1.cos(), 0.0, 0.0, rotation.1.sin());
    // let rotation = &pitch * &yaw;
    // let rotation_prim = rotation.inverse();

    // 2. Instantiate renderer.
    let camera_default = Camera {
        resolution: args.resolution.unwrap_or((64, 64)),
        ..Default::default()
    };
    let mut renderer = TerminalBuilder::default()
        .with_camera(camera_default.clone())
        .expect("Bad option config.")
        .build()
        .unwrap();

    let mut extras = renderer.extras().clone();
    extras.pixel_width_scaling = 65.5 / 43.5;
    renderer.set_extras(extras);

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

    // 5. Create a state handler.
    let mut state = StateHandler::new(args, event_handler, vertices, line_draw_order);

    // 6. Engine loop
    while state.event_handler.running() {
        let updated_config = state.update(renderer.config().clone());
        let mut writer = BufWriter::new(stdout().lock());

        let banner_text = "GRPE";
        let banner_fill_width =
            (updated_config.camera.resolution.0 as usize - banner_text.len()) / 2 - 1; // Note: "-1" for extra space(s).
        let banner_char = "=";
        let banner = banner_char.repeat(banner_fill_width);
        write!(writer, "\x1B[H\x1B[2K").unwrap();
        write!(
            writer,
            "\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner} {banner_text} {banner}\x1B[0m"
        )
        .unwrap();
        if updated_config.camera.resolution.0 % 2 != 0 {
            // Just make it nice even if odd.
            write!(
                writer,
                "\x1B[1;38;2;0;0;0;48;2;255;255;0m{banner_char}\x1B[0m"
            )
            .unwrap();
        }

        if let ProjectionMode::Perspective { fov } = updated_config.camera.projection_mode {
            if state.args.info.is_some() {
                write!(
                    writer,
                    "\x1B[{};H\x1B[2K",
                    (updated_config.camera.resolution.1 + 4) / 2
                )
                .unwrap();
                let info  = format!("FPS: {:0>4} | Events handled: {:0>10} | Resolution: ({},{}) | FOV: {:0>3} | Camera Rotation: (Pitch: {:.2}, Yaw: {:.2}) | Camera Position: ({:.2},{:.2},{:.2}) | ViewMode: {} | RenderOption: {}",
                    state.info().fps_smoothened,
                    state.info().event_count,
                    updated_config.camera.resolution.0, updated_config.camera.resolution.1, fov,
                    state.info().rotation.0, state.info().rotation.1,
                    updated_config.camera.position[0],  updated_config.camera.position[1], updated_config.camera.position[2],
                    state.info().view_mode, state.info().render_option,
                );
                write!(
                    writer,
                    "{}",
                    info.chars()
                        .take(updated_config.camera.resolution.0 as usize)
                        .collect::<String>()
                )
                .unwrap();
            }
        }

        writer.flush().unwrap();
        drop(writer);

        if state.info().invert_colors {
            println!("\x1B[H\x1B[7m"); // Invert colors. (Move to first row before printing/receiving, because it will be cleared anyway.)
        }

        renderer = renderer
            .set_config(updated_config)
            .expect("Bad configuration.");
        renderer.render();

        println!("\x1B[H\x1B[0m"); // Restore style . (Move to first row before printing/receiving, because it will be cleared anyway.)
    }

    std::thread::sleep(std::time::Duration::from_millis(100));
}
