use swr_core::ColorBuffer;

pub trait State {
    fn render(&mut self, buffer: ColorBuffer);
}

impl State for () {
    fn render(&mut self, _buffer: ColorBuffer) {}
}
