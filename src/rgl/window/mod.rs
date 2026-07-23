pub mod err;

use glfw::{WindowEvent, Glfw, GlfwReceiver, Context};
use glfw::ffi::{glfwSetFramebufferSizeCallback, GLFWwindow};
use crate::rgl::utils::gl_get_string;
use crate::rgl::window::err::WindowError;

pub struct Window<'a, T> {
    width: u32,
    height: u32,
    title: &'a str,
    window: glfw::PWindow,
    window_receiver: GlfwReceiver<(f64, WindowEvent)>,
    event_handler: T
}

impl<'a, T> Window<'a, T>
{
    pub fn new(glfw: &mut Glfw, width: u32, height: u32, title: &'a str, event_handler: T) -> Result<Window<'a, T>, WindowError> {
        let win_opt = glfw.create_window(width, height, title, glfw::WindowMode::Windowed);
        let Some((mut win, win_receiver)) = win_opt else {
            return Err(WindowError::FailedToCreateWindow);
        };

        win.make_current();
        win.set_key_polling(true);

        // load OpenGL function pointers using GLFW's loader function
        gl::load_with(|s| {
            win.get_proc_address(s).map_or_else(
                || {
                    eprintln!("Unsupported GL function: {s}");
                    std::ptr::null()
                },
                |f| f as *const _,
            )
        });

        // set viewport and clear color
        unsafe {
            gl::Viewport(0, 0, 800, 600);
            gl::ClearColor(0.2, 0.0, 0.2, 1.0);
        }

        println!("OpenGL version: {}", gl_get_string(gl::VERSION));
        println!("GLSL version: {}", gl_get_string(gl::SHADING_LANGUAGE_VERSION));

        win.set_framebuffer_size_callback(| _, width, height| {
            println!("GL framebuffer resized to : {}x{}", width, height);
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        });

        Ok(
            Window {
                width,
                height,
                title,
                window: win,
                window_receiver: win_receiver,
                event_handler
            }
        )
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }

    pub fn poll_events<S>(&mut self, glfw: &mut Glfw, state: &mut S)
    where
        T: FnMut(&mut glfw::Window, WindowEvent, &mut S)
    {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.window_receiver) {
            (self.event_handler)(&mut *self.window, event, state);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}