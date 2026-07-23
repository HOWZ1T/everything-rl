pub mod rgl;

use gl::types::{GLsizei, GLsizeiptr, GLuint, GLvoid};
use glfw;
use glfw::{WindowEvent};
use rgl::AppCallbacks;

struct Triangle {
    vertices: [f32; 9],
    indices: [u32; 3],
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl Triangle {
    pub fn new() -> Self {
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0,  0.5, 0.0
        ];
        let indices: [u32; 3] = [
            0, 1, 2,
        ];
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            // generate
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            // bind
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // set vertex attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * size_of::<f32>() as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Triangle {
            vertices,
            indices,
            vao,
            vbo,
            ebo
        }
    }
}

struct AppState {
    counter: i32,
    t0: f64,
    t1: f64,
    triangle: Triangle,
}

struct MyApp;

impl AppCallbacks for MyApp {
    type State = AppState;

    fn init(&mut self) -> AppState {
        AppState { counter: 0, t0: 0.0, t1: 0.0, triangle: Triangle::new() }
    }

    fn render(&mut self, state: &mut AppState) {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::BindVertexArray(state.triangle.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    fn event(&mut self, window: &mut glfw::Window, event: WindowEvent, state: &mut AppState) {

    }

    fn update(&mut self, state: &mut AppState, delta_ms: f64) {
        state.t1 += delta_ms;
        if state.t1 - state.t0 >= 1000.0 {
            state.t0 = 0.0;
            state.t1 = 0.0;
            state.counter += 1;
            println!("counter: {}", state.counter);
        }
    }
}

fn main() {
    let res = rgl::App::new(800, 600, "Everything RL", MyApp);
    if res.is_err() {
        panic!("{:?}", res.err().unwrap());
    }
    let mut app = res.ok().unwrap();
    app.set_clear_color([1.0, 0.0, 1.0, 1.0]);
    app.run().expect("TODO: panic message");
}
