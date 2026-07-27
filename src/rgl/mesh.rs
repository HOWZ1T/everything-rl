use gl::types::{GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};

#[derive(Debug)]
pub enum MeshError {
    InvalidVertexAttribs(String),
}

pub struct Raw {}
pub struct Compiled {}

pub struct GlHandles {
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
}

impl Drop for GlHandles {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
        println!("dropped GlHandles")
    }
}

pub struct Mesh<State> {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vert_attribs: Vec<GLuint>,
    pub gl: Option<GlHandles>,
    _marker: std::marker::PhantomData<State>,
}

impl<State> Mesh<State> {
    pub fn validate(vertices: &Vec<f32>, vert_attribs: &Vec<GLuint>) -> Result<(), MeshError> {
        let total_stride: GLuint = vert_attribs.iter().sum();
        if total_stride == 0 {
            return Err(MeshError::InvalidVertexAttribs(
                "Total vertex stride cannot be 0".to_string(),
            ));
        }

        if vertices.len() as GLuint % total_stride != 0 {
            return Err(MeshError::InvalidVertexAttribs(format!(
                "Vertices does not align with vertex stride: {}",
                total_stride
            )));
        }

        Ok(())
    }
}

impl Mesh<Raw> {
    pub fn new(
        vertices: Vec<f32>,
        indices: Vec<u32>,
        vert_attribs: Vec<GLuint>,
    ) -> Result<Mesh<Raw>, MeshError> {
        Self::validate(&vertices, &vert_attribs)?;

        Ok(Mesh {
            vertices,
            indices,
            vert_attribs,
            gl: None,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn compile(self) -> Result<Mesh<Compiled>, MeshError> {
        Self::validate(&self.vertices, &self.vert_attribs)?;
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;
        let stride: usize = self.vert_attribs.iter().sum::<GLuint>() as usize * size_of::<f32>();

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
                (self.vertices.len() * size_of::<f32>()) as GLsizeiptr,
                self.vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * size_of::<u32>()) as GLsizeiptr,
                self.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // set vertex attributes
            let mut offset: usize = 0;
            for (i, vert_attrib_size) in self.vert_attribs.iter().enumerate() {
                gl::VertexAttribPointer(
                    i as GLuint,
                    *vert_attrib_size as GLint,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as GLsizei,
                    offset as *const GLvoid,
                );
                gl::EnableVertexAttribArray(i as GLuint);
                offset += *vert_attrib_size as usize * size_of::<f32>();
            }

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Ok(Mesh {
            vertices: self.vertices,
            indices: self.indices,
            vert_attribs: self.vert_attribs,
            gl: Some(GlHandles { vao, vbo, ebo }),
            _marker: std::marker::PhantomData,
        })
    }
}

impl Mesh<Compiled> {}
