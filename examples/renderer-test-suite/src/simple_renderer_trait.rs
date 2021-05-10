/// Used for implementations of your renderer.
///
/// Your conformance test suite is used to verify that an implementations of `SimpleRenderer`
/// work as expected.
///
/// You might then go on to implement your renderer on top of different backends, such as Metal,
/// Vulkan, WebGPU, WebGL, or perhaps even a CPU based implementation.
pub trait SimpleRenderer {
    /// Process a series of [`RenderCommand`]s.
    fn render(&mut self, render_commands: &[RenderCommand]);

    /// Return all of the RGBA pixels from the simple renderer's pixel buffer.
    ///
    /// Note that a real renderer might expose ways to allow you to read back bytes from a
    /// particular subregion of a particular texture, rather than only having a single buffer of
    /// bytes to read back from.
    fn rgba_pixels(&self) -> Vec<u8>;
}

/// A real renderer might have a much more complex RenderGraph as its input, but our simple render
/// just takes some basic commands.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum RenderCommand {
    /// Create an all red pixel buffer.
    AllRed,
    /// Create an all blue pixel buffer.
    AllBlue,
}
