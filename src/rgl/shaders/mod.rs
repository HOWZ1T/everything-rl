pub mod shader;
pub mod shader_program;
mod uniforms;

pub use shader::{Shader, ShaderError, ShaderType};
pub use shader_program::{ShaderProgram, ShaderProgramError};