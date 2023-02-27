pub trait Screen {
    fn write_scanline(&mut self);
    fn swap_buffers(&mut self);
}

pub struct StubScreen;

impl Screen for StubScreen {
    fn write_scanline(&mut self) {}
    fn swap_buffers(&mut self) {}
}
