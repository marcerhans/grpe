use crate::{Camera, RenderOption, RendererBuilderTrait, RendererConfiguration, RendererTrait, __RendererTrait};

#[derive(Default)]
struct TerminalBuilder {
    config: RendererConfiguration,
}

impl RendererBuilderTrait for TerminalBuilder {
    type Renderer = Terminal;

    fn with_camera(mut self, camera: Camera) -> Self {
        self.config.camera = camera;
        self
    }

    fn with_option(mut self, option: RenderOption) -> Self {
        self.config.option = option;
        self
    }

    fn build(self) -> Self::Renderer {
        Self::Renderer::new(self.config)
    }

    fn build_with_config(self, config: RendererConfiguration) -> Self::Renderer {
        Self::Renderer::new(config)
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
    use crate::{RendererConfiguration, RendererTrait};
    use super::Terminal;
}