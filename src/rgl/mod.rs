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

pub struct App<'a, T, U, W, S> {
    glfw: Glfw,
    window: Window<'a, U, S>,
    render: T,
    update: W,
    clear_color: [f32; 4],
    state: S,
}

impl<'a, T, U, W, S> App<'a, T, U, W, S>
where
    T: FnMut(&mut S),
    U: FnMut(&mut glfw::Window, WindowEvent, &mut S),
    W: FnMut(&mut S, f64)
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

    pub fn new(width: u32, height: u32, title: &'a str, state: S, render: T, event_handler: U, update: W) -> Result<App<'a, T, U, W, S>, AppError> {
        let res = Self::init();
        if res.is_err() {
            return Err(res.err().unwrap())
        }
        let mut glfw = res?;

        let res: Result<Window<'a, U, S>, WindowError> = Window::new(&mut glfw, width, height, title, event_handler);
        if res.is_err() {
            return Err(AppError::WindowError(res.err().unwrap()))
        }

        Ok(
            App {
                glfw,
                window: res.unwrap(),
                render,
                update,
                clear_color: [0.0, 0.0, 0.0, 1.0],
                state,
            }
        )
    }

    pub fn set_clear_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.clear_color = color;
        unsafe {
            gl::ClearColor(
                self.clear_color[0],
                self.clear_color[1],
                self.clear_color[2],
                self.clear_color[3],
            );
        }
        self
    }

    pub fn run(mut self) -> Result<(), AppError> {
        let mut t0 = self.glfw.get_time();
        while !self.window.should_close() {
            self.window.poll_events(&mut self.glfw, &mut self.state);
            
            // update
            (self.update)(&mut self.state, (self.glfw.get_time() - t0) * 1000.0);
            t0 = self.glfw.get_time();

            // render
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                (self.render)(&mut self.state);
            }

            // draw to screen
            self.window.swap_buffers();
        }
        Ok(())
    }
}