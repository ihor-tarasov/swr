use std::{num::NonZeroU32, rc::Rc};

use swr_core::Frame;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, OwnedDisplayHandle},
    window::{Window, WindowId},
};

use crate::{Config, State, WindowError};

type SurfaceState = softbuffer::Surface<OwnedDisplayHandle, Rc<Window>>;

struct WindowState {
    window: Rc<Window>,
    context: softbuffer::Context<OwnedDisplayHandle>,
    surface_resized: bool,
    depth_buffer: Option<Vec<f32>>,
}

impl WindowState {
    fn new(event_loop: &ActiveEventLoop, config: &Config) -> Result<Self, WindowError> {
        let context = softbuffer::Context::new(event_loop.owned_display_handle())?;
        let attributes = Window::default_attributes()
            .with_title(config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height));
        let window = Rc::new(
            event_loop
                .create_window(attributes)
                .map_err(|error| WindowError::CreateWindow(error))?,
        );
        let depth_buffer = if config.depth_test {
            Some(vec![0.0; config.width as usize * config.height as usize])
        } else {
            None
        };

        Ok(Self {
            window,
            context,
            surface_resized: false,
            depth_buffer,
        })
    }

    fn create_surface(&self) -> Result<SurfaceState, WindowError> {
        Ok(softbuffer::Surface::new(
            &self.context,
            self.window.clone(),
        )?)
    }

    fn window_event<S>(
        &mut self,
        event_loop: &ActiveEventLoop,
        surface: &mut Option<SurfaceState>,
        window_id: WindowId,
        event: WindowEvent,
        state: &mut S,
    ) -> Result<(), WindowError>
    where
        S: State,
    {
        if self.window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => Ok(event_loop.exit()),
                WindowEvent::Resized(size) => {
                    if let Some(surface) = surface {
                        if let (Some(width), Some(height)) =
                            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                        {
                            surface.resize(width, height)?;
                            self.surface_resized = true;
                        }
                    }
                    Ok(())
                }
                WindowEvent::RedrawRequested => {
                    if let (Some(surface), true) = (surface, self.surface_resized) {
                        let mut buffer = surface.buffer_mut()?;
                        let size = self.window.inner_size();
                        let color_buffer = Frame::new(
                            size.width,
                            size.height,
                            &mut buffer,
                            self.depth_buffer.as_deref_mut(),
                        );
                        state.render(color_buffer);
                        buffer.present()?;
                    }
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    fn about_to_wait(&self) {
        self.window.request_redraw();
    }
}

pub struct App<'a, S> {
    window_state: Option<WindowState>,
    surface_state: Option<SurfaceState>,
    config: Config<'a>,
    result: Result<(), WindowError>,
    state: S,
}

impl<'a, S> App<'a, S>
where
    S: State,
{
    pub fn new(config: Config<'a>, state: S) -> Self {
        Self {
            window_state: None,
            surface_state: None,
            config,
            result: Ok(()),
            state,
        }
    }

    pub fn result(self) -> Result<(), WindowError> {
        self.result
    }
}

impl<'a, S> ApplicationHandler for App<'a, S>
where
    S: State,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_state.is_none() {
            self.window_state = match WindowState::new(event_loop, &self.config) {
                Ok(window_state) => Some(window_state),
                Err(error) => {
                    self.result = Err(error);
                    event_loop.exit();
                    return;
                }
            }
        }
        if self.surface_state.is_none() {
            self.surface_state = match self.window_state.as_ref().unwrap().create_surface() {
                Ok(surface_state) => Some(surface_state),
                Err(error) => {
                    self.result = Err(error);
                    event_loop.exit();
                    return;
                }
            }
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.surface_state = None;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window_state) = self.window_state.as_mut() {
            match window_state.window_event(
                event_loop,
                &mut self.surface_state,
                window_id,
                event,
                &mut self.state,
            ) {
                Ok(_) => {}
                Err(error) => {
                    self.result = Err(error);
                    event_loop.exit();
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window_state) = self.window_state.as_ref() {
            window_state.about_to_wait();
        }
    }
}
