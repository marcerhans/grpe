use crate::{Camera, RenderOption, RendererBuilderTrait, RendererConfiguration, RendererTrait, __RendererTrait};

#[derive(Default)]
struct TerminalBuilder;

impl TerminalBuilder {}

impl RendererBuilderTrait for TerminalBuilder {
    type Renderer = Terminal;

    fn with_camera(self, camera: Camera) -> Self {
        todo!()
    }

    fn with_option(self, option: RenderOption) -> Self {
        todo!()
    }

    fn build(self) -> Self::Renderer {
        todo!()
    }

    fn build_with_config(self, config: RendererConfiguration) -> Self::Renderer {
        todo!()
    }
}

struct Terminal {
    // config: RendererConfiguration,
}

impl Terminal {}

impl RendererTrait for Terminal {
    fn config(&self) -> RendererConfiguration {
        // self.config.clone()
        RendererConfiguration::default()
    }

    fn set_config(&mut self, config: RendererConfiguration) -> Result<(), &'static str> {
        // self.config = config;
        Ok(())
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

impl __RendererTrait for Terminal {
    fn new(config: RendererConfiguration) -> Self {
        Terminal { }
    }
}

#[cfg(test)]
mod tests {
    use crate::{RendererConfiguration, RendererTrait, __RendererTrait};
    use super::Terminal;

    #[test]
    fn main() {
        let config = RendererConfiguration::default();
        let renderer: Box<dyn RendererTrait> = Box::new(Terminal::new(config));
    }
}