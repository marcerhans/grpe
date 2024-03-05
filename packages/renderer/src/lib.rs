use std::cell::RefMut;

use linear_algebra::matrix::Matrix;

pub mod strategy;
pub use strategy::renderer;

// pub trait VertexTrait<'a> {
//     type Output;
//     fn position(self) -> Self::Output;
// }

// pub trait CanvasTrait<'a> : VertexTrait<'a> {
//     fn parameters(self) -> (&'a Matrix, &'a Matrix);
// }

// struct Foo {
//     matrix: Matrix,
//     u: u32,
// }

// impl<'a> VertexTrait<'a> for &'a Foo {
//     type Output = &'a u32;
//     fn position(self) -> Self::Output{
//         println!("what");
//         &self.u
//     }
// }

// impl<'a> VertexTrait<'a> for &'a mut Foo {
//     type Output = &'a u32;
//     fn position(self: Self) -> Self::Output {
//         println!("no");
//         &mut self.u
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn main() {
//         let mut foo = Foo {
//             matrix: Matrix::from_slices(&[
//                 &[1.0, 2.0, 3.0],
//             ]),
//             u: 8,
//         };
//         let bar = &mut foo;

//         let mut what: &mut u32 = bar.position();
//         (*what) += 2;
//         // println!("{:?}", (&mut foo as VertexTrait<'_, _>).position());
//     }
// }

// pub trait DimensionsTrait {
//     fn width(&self) -> usize;
//     fn height(&self) -> usize;
// }

// /// [RendererBuilderTrait] are categorized settings and initial values for a renderer ([RendererTrait]).
// pub trait RendererBuilderTrait {
//     type Dimensions: DimensionsTrait;
//     type Camera: VertexTrait;
//     type Canvas: CanvasTrait;

//     fn new() -> Self;
//     fn dimensions(self, dimensions: Self::Dimensions) -> Self;
//     fn camera(self, camera: Self::Camera) -> Self;
//     fn canvas(self, canvas: Self::Canvas) -> Self;
//     // fn build(self) -> renderer::Renderer;
// }

// /// [RendererTrait] for rendering to display.
// pub trait RendererTrait {
//     type Vertex: VertexTrait;

//     /// Project vertices on to [CanvasTrait].
//     fn project(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]);

//     /// Rasterize [RendererTrait::project]ed vertices.
//     fn rasterize(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]);

//     /// Do all steps needed, in correct order, to produce a fully rendered image.
//     fn run_pipeline(&self, vertices: &[(Self::Vertex, Self::Vertex, Self::Vertex)]) {
//         self.project(vertices);
//         self.rasterize(vertices);
//     }
// }