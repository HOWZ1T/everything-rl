use glfw::{Glfw, WindowEvent};
use crate::rgl::window::err::WindowError;
use crate::rgl::window::Window;

pub mod window;
pub mod utils;

#[derive(Debug)]
pub enum AppError {
    InitError(glfw::InitError),
    WindowError(WindowError),
}

pub struct App<'a, T, U> {
    glfw: Glfw,
    window: Window<'a, U>,
    render: T
}

impl<'a, T, U> App<'a, T, U> 
where
    T: FnMut(),
    U: FnMut(&mut glfw::Window, WindowEvent)
{
    fn init() -> Result<Glfw, AppError> {
        let res = glfw::init_no_callbacks();
        if res.is_err() {
            return Err(AppError::InitError(res.err().unwrap()))
        }

        let mut glfw = res.unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        Ok(glfw)
    }

    pub fn new(width: u32, height: u32, title: &'a str, render: T, event_handler: U) -> Result<App<'a, T, U>, AppError> {
        let res = Self::init();
        if res.is_err() {
            return Err(res.err().unwrap())
        }
        let mut glfw = res?;

        let res = Window::new(&mut glfw, width, height, title, event_handler);
        if res.is_err() {
            return Err(AppError::WindowError(res.err().unwrap()))
        }

        Ok(
            App {
                glfw,
                window: res.unwrap(),
                render
            }
        )
    }
    
    pub fn run(mut self) -> Result<(), AppError> {
        while !self.window.should_close() {
            self.window.poll_events(&mut self.glfw);

            // render
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                (self.render)();
            }

            // draw to screen
            self.window.swap_buffers();
        }
        Ok(())
    }
}