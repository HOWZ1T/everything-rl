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

pub trait AppCallbacks {
    type State;

    // called once after the GL context has been created and function pointers loaded.
    fn init(&mut self) -> Self::State;
    fn render(&mut self, state: &mut Self::State);
    fn event(&mut self, window: &mut glfw::Window, event: WindowEvent, state: &mut Self::State);
    fn update(&mut self, state: &mut Self::State, delta_ms: f64);
}

pub struct App<'a, C: AppCallbacks> {
    glfw: Glfw,
    window: Window<'a>,
    callbacks: C,
    clear_color: [f32; 4],
    state: C::State,
}

impl<'a, C: AppCallbacks> App<'a, C> {
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

    pub fn new(width: u32, height: u32, title: &'a str, mut callbacks: C) -> Result<App<'a, C>, AppError> {
        let res = Self::init();
        if res.is_err() {
            return Err(res.err().unwrap())
        }
        let mut glfw = res?;

        let res: Result<Window<'a>, WindowError> = Window::new(&mut glfw, width, height, title);
        if res.is_err() {
            return Err(AppError::WindowError(res.err().unwrap()))
        }

        // GL context is live and function pointers are loaded past this point,
        // so it's safe for callbacks to create GL objects here.
        let state = callbacks.init();

        Ok(
            App {
                glfw,
                window: res.unwrap(),
                callbacks,
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
            {
                let App { window, glfw, state, callbacks, .. } = &mut self;
                window.poll_events(glfw, |w, event| callbacks.event(w, event, state));
            }

            // update
            let t1 = self.glfw.get_time();
            self.callbacks.update(&mut self.state, (t1 - t0) * 1000.0);
            t0 = t1;

            // render
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                self.callbacks.render(&mut self.state);
            }

            // draw to screen
            self.window.swap_buffers();
        }
        Ok(())
    }
}
