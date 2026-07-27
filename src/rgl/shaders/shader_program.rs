use std::collections::HashMap;
use gl::types::{GLchar, GLint, GLuint};
use crate::rgl::shaders::shader::{Shader, ShaderError};
use crate::rgl::shaders::shader_program::ShaderProgramError::ShaderProgramAlreadyLinked;
use crate::rgl::shaders::uniforms::{Uniform, UniformError};

#[derive(Debug)]
pub enum ShaderProgramError {
    ShaderAlreadyAttached(String),  // TODO since ShaderProgram owns shaders via take_shader this class of error might not be relevant
    ShaderProgramAlreadyLinked,
    FailedToGetShaderId(String),
    ShaderError(ShaderError),
    LinkError(String),
    UnsupportedShaderType(String),
    UniformError(UniformError),
}

#[derive(Clone, Copy, Debug)]
pub struct ShaderProgramCapabilities {
    max_num_vertex_attribs: GLint,
}

impl ShaderProgramCapabilities {
    pub fn new() -> ShaderProgramCapabilities {
        let mut max_num_vertex_attribs: GLint = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_num_vertex_attribs);
        }

        ShaderProgramCapabilities {
            max_num_vertex_attribs
        }
    }

    pub fn max_num_vertex_attribs(&self) -> GLint {
        self.max_num_vertex_attribs
    }
}

pub struct ShaderProgram {
    pub name: String,
    id: Option<GLuint>,
    shaders: HashMap<String, Shader>,
    has_linked: bool,
    capabilities: ShaderProgramCapabilities,
}

impl ShaderProgram {
    pub fn new(name: String) -> ShaderProgram {
        ShaderProgram {
            name,
            id: None,
            shaders: HashMap::new(),
            has_linked: false,
            capabilities: ShaderProgramCapabilities::new(),
        }
    }

    pub fn has_shader_by_name(&self, name: &str) -> bool {
        self.shaders.contains_key(name)
    }

    pub fn has_shader(&self, shader: &Shader) -> bool {
        self.has_shader_by_name(shader.name.as_str())
    }

    pub fn take_shader(&mut self, shader: Shader) -> Result<(), ShaderProgramError> {
        if self.has_linked {
            return Err(ShaderProgramAlreadyLinked);
        }

        if self.has_shader(&shader) {
            return Err(ShaderProgramError::ShaderAlreadyAttached(shader.name.clone()));
        }

        self.shaders.insert(shader.name.clone(), shader);
        Ok(())
    }

    pub fn link(&mut self) -> Result<(), ShaderProgramError> {
        if self.has_linked {
            return Err(ShaderProgramAlreadyLinked);
        }

        let mut err_msg = None;
        unsafe {
            self.id = Some(gl::CreateProgram());

            for (shader_name, shader) in &self.shaders {
                let shader_id = shader.id();
                if shader_id.is_none() {
                    return Err(ShaderProgramError::FailedToGetShaderId(shader_name.clone()));
                }
                let shader_id = shader_id.unwrap();
                println!("attaching shader {}...", shader_name);
                gl::AttachShader(self.id.unwrap(), shader_id);
            }

            gl::LinkProgram(self.id.unwrap());

            // check for link error
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(self.id.unwrap(), gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let cap = 1024;
                let mut info_log = Vec::with_capacity(cap);
                info_log.set_len(cap);

                gl::GetProgramInfoLog(self.id.unwrap(), 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                err_msg = Some(String::from_utf8_lossy(&info_log).to_string());
            }
        }

        if err_msg.is_some() {
            return Err(ShaderProgramError::LinkError(err_msg.unwrap()));
        }

        self.has_linked = true;
        Ok(())
    }

    pub fn compile_and_link(&mut self) -> Result<(), ShaderProgramError> {
        if self.has_linked {
            return Err(ShaderProgramAlreadyLinked);
        }

        for (_shader_name, shader) in &mut self.shaders {
            println!("compiling {}...", _shader_name);
            let result = shader.compile();
            if result.is_err() {
                return Err(ShaderProgramError::ShaderError(result.unwrap_err()));
            }
        }

        self.link()?;
        Ok(())
    }

    pub fn id(&self) -> GLuint {
        self.id.expect("Failed to get shader program id via .id()")
    }

    pub fn get_capabilities(&self) -> ShaderProgramCapabilities {
        self.capabilities
    }

    pub fn set_uniform<T>(&self, name: &str, value: T) -> Result<(), ShaderProgramError>
    where
        Self: Uniform<T>,
    {
        if !self.has_linked {
            return Err(ShaderProgramAlreadyLinked);
        }

        let res = Uniform::set_uniform(self, name, value);
        if res.is_err() {
            return Err(ShaderProgramError::UniformError(res.unwrap_err()));
        }
        Ok(())
    }
}