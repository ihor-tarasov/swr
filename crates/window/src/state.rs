use swr_core::Frame;

pub trait State {
    fn render(&mut self, buffer: Frame);
}

impl State for () {
    fn render(&mut self, _buffer: Frame) {}
}
