use crate::{RendererBuilderTrait, RendererTrait, __RendererTrait};

#[derive(Default)]
struct TerminalBuilder;

impl TerminalBuilder {}

impl RendererBuilderTrait<TerminalBuilder> for TerminalBuilder {
    type Renderer = Terminal;

    fn with_camera(self, camera: crate::Camera) -> TerminalBuilder {
        todo!()
    }

    fn with_option(self, option: crate::RenderOption) -> TerminalBuilder {
        todo!()
    }

    fn build(self) -> Self::Renderer {
        todo!()
    }

    fn build_with_config(self, config: crate::RendererConfiguration) -> Self::Renderer {
        todo!()
    }
}

struct Terminal {}

impl Terminal {}

impl RendererTrait for Terminal {
    fn config(&self) -> crate::RendererConfiguration {
        todo!()
    }

    fn set_config(&mut self, config: crate::RendererConfiguration) -> Result<(), &'static str> {
        todo!()
    }

    fn set_vertices(&mut self, vertices: &[linear_algebra::vector::VectorRow<f64, 3>]) {
        todo!()
    }

    fn set_vertices_line_draw_order(&mut self, order: &[&[usize]]) {
        todo!()
    }

    fn render(&self) {
        todo!()
    }
}

impl __RendererTrait<Terminal> for Terminal {
    fn new(config: crate::RendererConfiguration) -> Terminal {
        todo!()
    }
}