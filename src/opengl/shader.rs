use std::ffi::CString;
use std::fmt;

use gl::types::{GLchar, GLint, GLuint};

// An OpenGL shader uniform location ID.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UniformLocation(pub(crate) i32);

// An OpenGL shader attribute location ID.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AttribLocation(pub(crate) i32);

// An OpenGL shader program ID.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ShaderProgram(pub(crate) GLuint);

impl ShaderProgram {
    /// Use this program.
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }

    /// The location of a uniform.
    pub fn get_uniform_location(&self, name: &str) -> Option<UniformLocation> {
        let s = CString::new(name).unwrap();
        let loc = unsafe { gl::GetUniformLocation(self.0, s.as_ptr()) };

        if loc < 0 {
            None
        } else {
            Some(UniformLocation(loc))
        }
    }

    /// The location of an attribute.
    pub fn get_attrib_location(&self, name: &str) -> Option<AttribLocation> {
        let s = CString::new(name).unwrap();
        let loc = unsafe { gl::GetAttribLocation(self.0, s.as_ptr()) };

        if loc < 0 {
            None
        } else {
            Some(AttribLocation(loc))
        }
    }
}

#[derive(Debug)]
pub struct ShaderCompileError(String);

impl fmt::Display for ShaderCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not compile shader: {}", self.0)
    }
}

/// Compiles a shader program composed of a vertex and fragment shader.
pub unsafe fn compile(vert: &str, frag: &str) -> Result<ShaderProgram, ShaderCompileError> {
    let program = gl::CreateProgram();

    let shader = gl::CreateShader(gl::VERTEX_SHADER);
    gl::ShaderSource(
        shader,
        1,
        &(vert.as_ptr() as *const GLchar),
        &(vert.len() as GLint),
    );
    gl::CompileShader(shader);
    verify_shader(shader)?;
    gl::AttachShader(program, shader);

    let shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    gl::ShaderSource(
        shader,
        1,
        &(frag.as_ptr() as *const GLchar),
        &(frag.len() as GLint),
    );
    gl::CompileShader(shader);
    verify_shader(shader)?;
    gl::AttachShader(program, shader);

    gl::LinkProgram(program);
    verify_program(program)?;

    Ok(ShaderProgram(program))
}

unsafe fn verify_shader(shader: GLuint) -> Result<(), ShaderCompileError> {
    let mut status = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

    if status == 1 {
        Ok(())
    } else {
        let shader_info_log = {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            if len > 0 {
                let mut log = String::with_capacity(len as usize);
                log.extend(std::iter::repeat('\0').take(len as usize));
                gl::GetShaderInfoLog(shader, len, &mut len, log[..].as_ptr() as *mut GLchar);
                log.truncate(len as usize);
                log
            } else {
                String::from("")
            }
        };

        Err(ShaderCompileError(shader_info_log))
    }
}

unsafe fn verify_program(program: GLuint) -> Result<(), ShaderCompileError> {
    let mut status = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    if status == 1 {
        Ok(())
    } else {
        let program_info_log = {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            if len > 0 {
                let mut log = String::with_capacity(len as usize);
                log.extend(std::iter::repeat('\0').take(len as usize));
                gl::GetProgramInfoLog(program, len, &mut len, log[..].as_ptr() as *mut GLchar);
                log.truncate(len as usize);
                log
            } else {
                String::from("")
            }
        };

        Err(ShaderCompileError(program_info_log))
    }
}
