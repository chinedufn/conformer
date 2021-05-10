//! An example of where we have a [`SimpleRenderer`] trait and we want to test implementers of
//! that trait.
//!
//! See main.rs for where we create and run the test suite.

#![deny(missing_docs)]

mod simple_renderer_trait;
pub use self::simple_renderer_trait::{RenderCommand, SimpleRenderer};

/// This is the type that we'll be testing in this example.
///
/// Your test suite might be used to test many different types that conform to some trait.
///
/// For example, you might write a test suite that can be used to test anything that implements
/// the [`SimpleRenderer`] trait, and then expose that to other people via a command line
/// application.
#[derive(Default)]
pub struct MySimpleRendererImplementation {
    width: u32,
    height: u32,
    rgba_pixel_buffer: Vec<u8>,
}

impl MySimpleRendererImplementation {
    /// Create a new simple renderer.
    ///
    /// Test cases in a [`conformer::TestSuite`] are independent, so each test case will create
    /// its own renderer.
    pub fn new(width: u32, height: u32) -> Self {
        MySimpleRendererImplementation {
            width,
            height,
            rgba_pixel_buffer: vec![255; width as usize * height as usize * 4],
        }
    }

    fn pixel_count(&self) -> usize {
        self.width as usize * self.height as usize
    }
}

impl SimpleRenderer for MySimpleRendererImplementation {
    fn render(&mut self, render_command: &[RenderCommand]) {
        let pixel_count = self.pixel_count();

        let color = match render_command.last().unwrap() {
            // Correct implementation. This will pass during in the test suite.
            RenderCommand::AllBlue => [0, 0, 255, 255],
            // Incorrect implementation. This will fail during the test suite.
            RenderCommand::AllRed => [11, 22, 33, 255],
        };

        self.rgba_pixel_buffer
            .copy_from_slice(&color.repeat(pixel_count));
    }

    fn rgba_pixels(&self) -> Vec<u8> {
        self.rgba_pixel_buffer.clone()
    }
}
