use std::env;

use renderer::RenderOption;

use crate::model::Model;

enum Arg {
    Help,
    Resolution,
    RenderOption,
    Info,
    Model,
    Fps,
}

#[derive(Default)]
pub struct Args {
    pub resolution: Option<(u64, u64)>,
    pub render_option: Option<RenderOption>,
    pub info: Option<()>,
    pub model: Option<Model>,
    pub fps: Option<u64>,
}

pub fn parse_args() -> Args {
    let args_raw: Vec<String> = env::args().skip(1).collect();
    let mut args = Args::default();
    let mut arg_it = args_raw.iter();

    while let Some(option) = arg_it.next() {
        let option = match option.as_str() {
            "-h" | "--help" => Arg::Help,
            "-r" | "--resolution" => Arg::Resolution,
            "-o" | "--option" => Arg::RenderOption,
            "-i" | "--info" => Arg::Info,
            "-m" | "--model" => Arg::Model,
            "-f" | "--fps" => Arg::Fps,
            _ => {
                println!("Unknown option \"{}\"", option);
                std::process::exit(1);
            }
        };

        match option {
            Arg::Help => {
                println!(
                    "GRPE Usage:
grpe [OPTION]

OPTIONS:
-h, --help
Default: false
Print this help section.

-r <width height>, --resolution <width height>
Default: 64 64
Set the resolution.

-o <option>, --render-option <option>
Default: vertices
Available options:
vertices - Renders only vertices.
wireframe - Renders only wireframe (lines between vertices).
wireframeandparticles - Wireframe + particles (single point vertices).
culling - Renders wireframe, but with backface culling.
cullingandparticles - Culling + particles (single point vertices).
polyfillandculling - Fill polygons + culling.
polyfillandcullingandparticles - Fill polygons + culling + particles (single point vertices).

-i, --info
Default: true
During execution, print additional information at the bottom.
This includes fps, missed frames, fov, etc.

-m, --model
Default: \"plane\"
What to display.
Available models:
plane
spiral

-f, --fps
Default: 60
Set the frames per second.
                    "
                );
                std::process::exit(0);
            }
            Arg::Resolution => {
                let width: u64 = arg_it.next().unwrap().parse().unwrap();
                let height: u64 = arg_it.next().unwrap().parse().unwrap();
                args.resolution = Some((width, height));
            }
            Arg::RenderOption => {
                let option: RenderOption = arg_it.next().unwrap().parse().unwrap();
                args.render_option = Some(option);
            }
            Arg::Info => {
                args.info = Some(());
            }
            Arg::Model => {
                let model = arg_it.next().unwrap().parse().unwrap_or_else(|_| {
                    println!("Unkown model type given. Please run with '-h' or '--help' option for full list.");
                    std::process::exit(0);
                });
                args.model = Some(model);
            }
            Arg::Fps => {
                let fps = arg_it.next().unwrap().parse().unwrap();
                args.fps = Some(fps);
            }
        }
    }

    args
}
