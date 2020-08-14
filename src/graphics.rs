extern crate gl;

use std;
use std::ffi::CString;

#[derive(Debug, Default)]
struct Shader {
    gl_handle: u32,
    shader_type: gl::types::GLenum,
}

impl Shader {
    fn create_shader(source: &str, kind: gl::types::GLenum) -> Shader {
        let id = unsafe{gl::CreateShader(kind)}; //Create shader and store it's handle in a variable
        unsafe{
            let csource = CString::new(source).expect("CString::new failed");
            gl::ShaderSource(id, 1, &csource.as_ptr(), std::ptr::null());//Put source shader code in shader
            gl::CompileShader(id);//compile shader code
        }
        //Handle potential error
        let mut success = gl::FALSE as gl::types::GLint;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            // gl::GetProgramiv(self.gl_handle, gl::LINK_STATUS, &mut success);
        }
        if success != gl::TRUE as gl::types::GLint{//get error string from opengl and display it
            // let mut len: gl::types::GLint = 0;
            unsafe {
                // gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let info_log = String::with_capacity(256);
                let mut error_size = 0i32;
                gl::GetShaderInfoLog(id, 512, &mut error_size, info_log.as_ptr() as _);
                println!("Error shader compilation failed with error: {:?} for: {:?}", info_log, id);
                panic!();
            }
        }
        Shader{gl_handle: id, shader_type: kind}
    }
}

impl Drop for Shader{
    fn drop(&mut self){
        unsafe {
            gl::DeleteShader(self.gl_handle);
        }
    }
}

#[derive(Debug, Default)]
struct ShaderProgram {
    gl_handle: u32
}

impl ShaderProgram {
//Creates a new program CreateProgram() must return an unsigned int
    pub fn new() -> ShaderProgram {
        ShaderProgram{
            gl_handle: unsafe{gl::CreateProgram()}
        }
    }

//Attach Shader to Program
    pub fn attach_shader(&mut self, shader: &Shader){
        unsafe {
            let t = &shader.shader_type;
            println!("Attaching shader of type {}, handle:{} to program {}", t, shader.gl_handle, self.gl_handle);
            gl::AttachShader(self.gl_handle, shader.gl_handle);
        }
    }

    pub fn link(&self){
        unsafe {
            //run the actual gl link function
            gl::LinkProgram(self.gl_handle);
            //the rest of this function is error handling
            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(self.gl_handle, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint{
                let info_log = String::with_capacity(256);
                let mut error_size = 0i32;
                gl::GetShaderInfoLog(self.gl_handle, 512, &mut error_size, info_log.as_ptr() as _);
                println!("Error link failed with error: {:?} for: {:?}", info_log, self.gl_handle);
                panic!();
            }
            else{
                println!("Linked successfully {}", self.gl_handle);
            }
        }
    }
    pub fn use_program(&self){
        unsafe {gl::UseProgram(self.gl_handle)}
    }
}

impl Drop for ShaderProgram{
    fn drop(&mut self){
        unsafe {
            gl::DeleteProgram(self.gl_handle);
        }
    }
}

macro_rules! create_new_shader {
    ($code:ident, $type:expr) => {
        Shader::create_shader($code, $type);
    };
}

pub fn do_graphics_stuff() -> u32{
    let vertex_code = "
    #version 330 core\n

    layout (location = 0) in vec3 Position;\n

    void main()
    {
        gl_Position = vec4(Position, 1.0);
    }
    ";
    let fragment_code = "
    #version 330 core\n

    out vec4 Color;\n

    void main()
    {
        Color = vec4(0.98f, 0.1f, 0.8f, 1.0f);
    }
    ";
    let vertex_shader = create_new_shader!(vertex_code, gl::VERTEX_SHADER);
    let fragment_shader = create_new_shader!(fragment_code, gl::FRAGMENT_SHADER);
    let mut shader_program = ShaderProgram::new();
    shader_program.attach_shader(&vertex_shader);
    shader_program.attach_shader(&fragment_shader);
    shader_program.link();
    shader_program.use_program();
//do buffer stuff
    let vertices: Vec<f32> = vec![
    -0.5, -0.5,
    0.5, -0.5,
    0.5, 0.5,
    -0.5, 0.5,
    ];
    let indices: Vec<u8> = vec![
    0, 1, 2,
    2, 3, 0
    ];
    let mut vbo: gl::types::GLuint = 0;
    let mut ibo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        //get indice data
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        gl::GenBuffers(1, &mut ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, // target
            (indices.len() * std::mem::size_of::<u8>()) as gl::types::GLsizeiptr, // size of data in bytes
            indices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            2, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (2 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );
        gl::BindVertexArray(0);
    }
    vao
}

pub fn draw(vao: u32){
    unsafe{
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::BindVertexArray(vao);
        // gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, std::ptr::null());
    }
}
