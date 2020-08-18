pub extern crate gl;

use std;
use std::ffi::CString;
use std::collections::HashMap;

// const vertex_number: u32 = gl::VERTEX_SHADER;
// const fragment_number: u32 = gl::FRAGMENT_SHADER;

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

// macro_rules! generate_shader_code {
//     (gl::VERTEX_SHADER) => {
//         let vertex_code = "
//         #version 330 core\n
//
//         layout (location = 0) in vec3 Position;\n
//
//         void main()
//         {
//             gl_Position = vec4(Position, 1.0);
//         }
//         ";
//         vertex_code
//     };
//     (gl::FRAGMENT_SHADER) => {
//         let fragment_code = "
//         #version 330 core\n
//
//         out vec4 Color;\n
//         uniform vec4 u_Color;\n
//         void main()
//         {
//             Color = u_Color;
//         }
//         ";
//         fragment_code
//     }
// }

macro_rules! create_new_shader {
    ($code:ident, $type:expr) => {
        Shader::create_shader($code, $type);
    };
}

// macro_rules! create_shaders {
//     ($program: ident, $type: expr,+) => {
//         let shader_code = generate_shader_code!($type);
//         let shader = create_new_shader!(shader_code, $type);
//         program.attach_shader(shader);
//     };
// }



pub struct Renderer {
    program: ShaderProgram,
    vertex_array_object: u32,
    index_buffer_object:u32,
    vertex_buffer_object: u32,
    uniform_cache: HashMap<String, i32>
}

impl Renderer {
    pub fn new() -> Renderer{
        Renderer{
            program: ShaderProgram::new(),
            vertex_array_object: 0,
            index_buffer_object: 0,
            vertex_buffer_object: 0,
            uniform_cache: HashMap::new()
        }
    }

    // fn attach_shaders(&self){
    //     create_shaders!(self.program, gl::VERTEX_SHADER, gl::FRAGMENT_SHADER);
    // }

    pub fn initialize_object_renderer(&mut self, vertices: Vec<f32>){
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
        uniform vec4 u_Color;\n
        void main()
        {
            Color = u_Color;
        }
        ";
        let vertex_shader = create_new_shader!(vertex_code, gl::VERTEX_SHADER);
        let fragment_shader = create_new_shader!(fragment_code, gl::FRAGMENT_SHADER);
        let mut shader_program = ShaderProgram::new();
        shader_program.attach_shader(&vertex_shader);
        shader_program.attach_shader(&fragment_shader);
        // self.attach_shaders();
        shader_program.link();
        let s = "u_Color";
        let location = unsafe {gl::GetUniformLocation(shader_program.gl_handle, s.as_ptr() as *const gl::types::GLbyte)};
        shader_program.use_program();
        unsafe {gl::Uniform4f(location, 0.2, 0.3, 0.8, 1.0);}



    //do buffer stuff
        let indices: Vec<u8> = vec![
        0, 4, 5,
        0, 1, 4,
        1, 3, 4,
        1, 2, 3,
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
                gl::DYNAMIC_DRAW, // usage
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
                gl::DYNAMIC_DRAW, // usage
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
        self.vertex_array_object = vao;
        self.index_buffer_object = ibo;
        self.vertex_buffer_object = vbo;

        // Renderer{program_id: shader_program.gl_handle, vertex_array_object: vao, index_buffer_object: ibo}
    }


    pub fn get_uniform_location(&mut self, slice: &str) -> i32{
        let name = slice.to_string();
        if self.uniform_cache.contains_key(&name) {
            let cached_location = *self.uniform_cache.get(&name).unwrap();
            if cached_location == -1 {
                println!("Warning cached uniform{:?} doesn't exist!", name);
            }
            cached_location
        }
        else{
            let location = unsafe {gl::GetUniformLocation(self.program.gl_handle, name.as_ptr() as *const gl::types::GLbyte)};
            if location == -1 {
                println!("Warnig new uniform: {:?} doesn't exist!", name);
            }
            self.uniform_cache.insert(name, location);
            location
        }
    }

    pub fn draw_object(&self, vertices: Vec<f32>){
        unsafe{
            gl::UseProgram(self.program.gl_handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::DYNAMIC_DRAW, // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // gl::Uniform4f(location, r, r, r, r);
            gl::BindVertexArray(self.vertex_array_object);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer_object);
            // gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}

pub fn do_graphics_stuff() -> [u32; 3]{
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
    uniform vec4 u_Color;\n
    void main()
    {
        Color = u_Color;
    }
    ";
    let vertex_shader = create_new_shader!(vertex_code, gl::VERTEX_SHADER);
    let fragment_shader = create_new_shader!(fragment_code, gl::FRAGMENT_SHADER);
    let mut shader_program = ShaderProgram::new();
    shader_program.attach_shader(&vertex_shader);
    shader_program.attach_shader(&fragment_shader);
    shader_program.link();
    let s = "u_Color";
    let location = unsafe {gl::GetUniformLocation(shader_program.gl_handle, s.as_ptr() as *const gl::types::GLbyte)};
    shader_program.use_program();
    unsafe {gl::Uniform4f(location, 0.2, 0.3, 0.8, 1.0);}



//do buffer stuff
    let vertices: Vec<f32> = vec![
    0.2, 0.4,
    0.4, 0.0,
    0.2, -0.4,
    -0.2, -0.4,
    -0.4, 0.0,
    -0.2, 0.4,
    ];
    let indices: Vec<u8> = vec![
    0, 4, 5,
    0, 1, 4,
    1, 3, 4,
    1, 2, 3,
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
    [vao, ibo, shader_program.gl_handle]
}

pub fn draw(draw_objects: [u32; 3], location: i32, r: f32){
    unsafe{
        gl::UseProgram(draw_objects[2]);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Uniform4f(location, r, r, r, r);

        gl::BindVertexArray(draw_objects[0]);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, draw_objects[1]);
        // gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_BYTE, std::ptr::null());
        gl::BindVertexArray(0);
    }
}
