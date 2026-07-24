use std::ffi::CString;
use gl::types::{GLchar, GLint, GLuint};

#[derive(Debug)]
pub enum ShaderError {
    CompileError(String),
}

pub enum ShaderType {
    Vertex,
    Fragment,
    Compute
}

pub struct Shader {
    pub name: String,
    pub source: CString,
    pub typ: ShaderType,
    id: Option<GLuint>,
    has_compiled: bool,

}

impl Shader {
    pub fn new(name: String, source: String, typ: ShaderType) -> Shader {
        let src = CString::new(source).expect("Failed to convert source to CString");
        Shader {
            name,
            source: src,
            typ,
            id: None,
            has_compiled: false,
        }
    }

    pub fn compile(&mut self) -> Result<(), ShaderError> {
        let gl_shader_type = match self.typ {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Compute => gl::COMPUTE_SHADER
        };

        let shader_id: GLuint;
        let mut err_msg: Option<String> = None;
        unsafe {
            shader_id = gl::CreateShader(gl_shader_type);
            gl::ShaderSource(shader_id, 1, &self.source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);

            // check for compile error
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let cap = 1024;
                let mut info_log = Vec::with_capacity(cap);
                info_log.set_len(cap);

                gl::GetShaderInfoLog(shader_id, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                err_msg = Some(String::from_utf8_lossy(&info_log).to_string());
            }
        }

        if err_msg.is_some() {
            return Err(ShaderError::CompileError(err_msg.unwrap()));
        }

        self.id = Some(shader_id);
        self.has_compiled = true;
        Ok(())
    }

    pub fn id(&self) -> Option<GLuint> {
        self.id
    }
}