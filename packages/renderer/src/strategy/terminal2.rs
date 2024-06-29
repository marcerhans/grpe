use crate::{Camera, RenderOption, RendererBuilderTrait, RendererConfiguration, RendererTrait, __RendererTrait};
use linear_algebra::vector::VectorRow;

#[allow(non_snake_case)]
mod PipelineStage {
    #![allow(non_upper_case_globals)]
    pub const Initialize: usize = 0;
}

#[derive(Default)]
struct TerminalBuilder {
    config: RendererConfiguration,
}

impl RendererBuilderTrait for TerminalBuilder {
    type Renderer = Terminal<{PipelineStage::Initialize}>;

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

/// Typed state terminal renderer.
struct Terminal<const STAGE: usize> {
    config: RendererConfiguration,
    vertices: Vec<VectorRow<f64, 3>>,
    // line_draw_order: Vec<usize>, // TODO
}

impl<const STAGE: usize> RendererTrait for Terminal<STAGE> {
    fn config(&self) -> RendererConfiguration {
        self.config.clone()
    }

    fn set_config(&mut self, config: RendererConfiguration) -> Result<(), &'static str> {
        self.config = config;
        Ok(())
    }

    fn set_vertices(&mut self, vertices: &[VectorRow<f64, 3>]) {
        self.vertices = vertices.to_owned();
    }

    fn set_vertices_line_draw_order(&mut self, order: &[&[usize]]) {
        todo!("Implement this later")
    }

    fn render(&self) {
        todo!()
    }
}

impl<const STAGE: usize> __RendererTrait for Terminal<STAGE> {
    fn new(config: RendererConfiguration) -> Self {
        Self {
            config,
            vertices: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{RendererConfiguration, RendererTrait};
    use super::Terminal;
}