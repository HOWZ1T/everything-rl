pub mod rgl;

use crate::rgl::shaders::ShaderType;
use gl::types::{GLsizei, GLsizeiptr, GLuint, GLvoid};
use glfw;
use glfw::WindowEvent;
use rgl::AppCallbacks;
use rgl::resource_manager::ResourceManager;
use rgl::shaders::{Shader, ShaderProgram};

struct Triangle {
    vertices: [f32; 18],
    indices: [u32; 3],
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl Triangle {
    pub fn new() -> Self {
        // interleaved: position (x, y, z), color (r, g, b)
        let vertices: [f32; 18] = [
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0,
            1.0,
        ];
        let indices: [u32; 3] = [0, 1, 2];
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

            // set vertex attributes
            let stride = 6 * size_of::<f32>() as GLsizei;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * size_of::<f32>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(1);

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Triangle {
            vertices,
            indices,
            vao,
            vbo,
            ebo,
        }
    }
}

struct AppState {
    counter: i32,
    t0: f64,
    t1: f64,
    triangle: Triangle,
    resource_manager: ResourceManager,
    shader_program: ShaderProgram,
}

struct MyApp;

impl AppCallbacks for MyApp {
    type State = AppState;

    fn init(&mut self) -> AppState {
        let resource_manager = ResourceManager::new();
        let Ok(frag_shader) = resource_manager.load_shader_source("resources/shaders/frag.glsl")
        else {
            panic!("Can't load frag shader");
        };

        let Ok(vert_shader) = resource_manager.load_shader_source("resources/shaders/vert.glsl")
        else {
            panic!("Can't load vert shader");
        };

        let frag_shader = Shader::new(
            "default_frag".parse().unwrap(),
            frag_shader,
            ShaderType::Fragment,
        );
        let vert_shader = Shader::new(
            "default_vert".parse().unwrap(),
            vert_shader,
            ShaderType::Vertex,
        );
        let mut shader_program = ShaderProgram::new("default program".parse().unwrap());
        let res = shader_program.take_shader(frag_shader);
        if res.is_err() {
            panic!(
                "Can't attach frag shader to program: {:?}",
                res.unwrap_err()
            );
        }
        let res = shader_program.take_shader(vert_shader);
        if res.is_err() {
            panic!(
                "Can't attach frag shader to program: {:?}",
                res.unwrap_err()
            );
        }
        let res = shader_program.compile_and_link();
        if res.is_err() {
            panic!("Can't compile shader program: {:?}", res.unwrap_err());
        }

        let capabilities = shader_program.get_capabilities();
        let max_num_vertex_attribs = capabilities.max_num_vertex_attribs();
        println!("max_num_vertex_attribs: {}", max_num_vertex_attribs);

        AppState {
            counter: 0,
            t0: 0.0,
            t1: 0.0,
            triangle: Triangle::new(),
            resource_manager,
            shader_program,
        }
    }

    fn render(&mut self, state: &mut AppState) {
        unsafe {
            gl::UseProgram(state.shader_program.id());
            gl::BindVertexArray(state.triangle.vao);
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    fn event(&mut self, window: &mut glfw::Window, event: WindowEvent, state: &mut AppState) {}

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
