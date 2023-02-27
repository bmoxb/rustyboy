pub trait Display {
    fn write_scanline(&mut self);
    fn swap_buffers(&mut self);
}

pub struct StubDisplay;

impl Display for StubDisplay {
    fn write_scanline(&mut self) {}
    fn swap_buffers(&mut self) {}
}
