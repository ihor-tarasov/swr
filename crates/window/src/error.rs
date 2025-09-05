use softbuffer::SoftBufferError;
use winit::error::{EventLoopError, OsError};

#[derive(Debug)]
pub enum WindowError {
    EventLoop(EventLoopError),
    CreateWindow(OsError),
    SoftBuffer(SoftBufferError),
}

impl From<EventLoopError> for WindowError {
    fn from(value: EventLoopError) -> Self {
        Self::EventLoop(value)
    }
}

impl From<SoftBufferError> for WindowError {
    fn from(value: SoftBufferError) -> Self {
        Self::SoftBuffer(value)
    }
}
