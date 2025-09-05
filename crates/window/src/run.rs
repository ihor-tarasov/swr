use winit::event_loop::EventLoop;

use crate::{Config, State, WindowError, app::App};

pub fn run<S>(config: Config, state: S) -> Result<(), WindowError>
where
    S: State,
{
    let event_loop = EventLoop::new()?;
    let mut app = App::new(config, state);
    event_loop.run_app(&mut app)?;
    app.result()
}
