pub mod rgl;

use crate::rgl::mesh;
use crate::rgl::mesh::Mesh;
use crate::rgl::shaders::ShaderType;
use glfw;
use glfw::WindowEvent;
use rgl::AppCallbacks;
use rgl::resource_manager::ResourceManager;
use rgl::shaders::{Shader, ShaderProgram};

struct Triangle {
    mesh: Mesh<mesh::Compiled>,
}

impl Triangle {
    pub fn new() -> Self {
        // interleaved: position (x, y, z), color (r, g, b)
        let vertices: [f32; 18] = [
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0,
            1.0,
        ];
        let indices: [u32; 3] = [0, 1, 2];

        let mesh = Mesh::new(Vec::from(vertices), Vec::from(indices), vec![3, 3]);

        if mesh.is_err() {
            panic!("{:?}", mesh.err());
        }

        let compiled_mesh = mesh.unwrap().compile();
        if compiled_mesh.is_err() {
            panic!("{:?}", compiled_mesh.err());
        }

        Triangle {
            mesh: compiled_mesh.unwrap(),
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
        let res = state.triangle.mesh.gl.as_ref();
        if res.is_none() {
            return;
        }
        let gl_handles = res.unwrap();
        unsafe {
            gl::UseProgram(state.shader_program.id());
            gl::BindVertexArray(gl_handles.vao);
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
