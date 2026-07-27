use std::ffi::CString;
use gl::types::{GLfloat, GLint, GLuint};
use crate::rgl::shaders::{ShaderProgram};
use crate::rgl::types::{IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};

#[derive(Debug)]
pub enum UniformError {
    CouldNotFindUniformLocation(String)
}

pub trait Uniform<T> {
    fn set_uniform(&self, name: &str, value: T) -> Result<(), UniformError>;
}

fn get_uniform_location(program: &ShaderProgram, name: &str) -> Result<GLint, UniformError> {
    let c_name = CString::new(name).expect("uniform name contained a null byte");
    let uniform_loc = unsafe { gl::GetUniformLocation(program.id(), c_name.as_ptr()) };
    if uniform_loc == -1 {
        return Err(UniformError::CouldNotFindUniformLocation(name.to_string()));
    }

    Ok(uniform_loc)
}

impl Uniform<GLint> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: GLint) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform1i(uniform_loc, value);
        }
        Ok(())
    }
}

impl Uniform<GLuint> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: GLuint) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform1ui(uniform_loc, value);
        }
        Ok(())
    }
}

impl Uniform<GLfloat> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: GLfloat) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform1f(uniform_loc, value);
        }
        Ok(())
    }
}

impl Uniform<bool> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: bool) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform1i(uniform_loc, value as GLint);
        }
        Ok(())
    }
}

impl Uniform<Vec2> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: Vec2) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform2f(uniform_loc, value.0, value.1);
        }
        Ok(())
    }
}

impl Uniform<Vec3> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: Vec3) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform3f(uniform_loc, value.0, value.1, value.2);
        }
        Ok(())
    }
}

impl Uniform<Vec4> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: Vec4) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform4f(uniform_loc, value.0, value.1, value.2, value.3);
        }
        Ok(())
    }
}

impl Uniform<IVec2> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: IVec2) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform2i(uniform_loc, value.0, value.1);
        }
        Ok(())
    }
}

impl Uniform<IVec3> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: IVec3) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform3i(uniform_loc, value.0, value.1, value.2);
        }
        Ok(())
    }
}

impl Uniform<IVec4> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: IVec4) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform4i(uniform_loc, value.0, value.1, value.2, value.3);
        }
        Ok(())
    }
}

impl Uniform<UVec2> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: UVec2) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform2ui(uniform_loc, value.0, value.1);
        }
        Ok(())
    }
}

impl Uniform<UVec3> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: UVec3) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform3ui(uniform_loc, value.0, value.1, value.2);
        }
        Ok(())
    }
}

impl Uniform<UVec4> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: UVec4) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::Uniform4ui(uniform_loc, value.0, value.1, value.2, value.3);
        }
        Ok(())
    }
}

// matrices are expected in column-major order, matching GLSL's own layout, so transpose is always GL_FALSE
impl Uniform<[f32; 4]> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: [f32; 4]) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::UniformMatrix2fv(uniform_loc, 1, gl::FALSE, value.as_ptr());
        }
        Ok(())
    }
}

impl Uniform<[f32; 9]> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: [f32; 9]) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::UniformMatrix3fv(uniform_loc, 1, gl::FALSE, value.as_ptr());
        }
        Ok(())
    }
}

impl Uniform<[f32; 16]> for ShaderProgram {
    fn set_uniform(&self, name: &str, value: [f32; 16]) -> Result<(), UniformError> {
        let uniform_loc = get_uniform_location(self, name)?;
        unsafe {
            gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, value.as_ptr());
        }
        Ok(())
    }
}