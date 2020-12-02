use glutin::{self, PossiblyCurrent};

use std::ffi::{CStr, CString};

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct Gl {
    pub gl: gl::Gl,
    pub program: gl::types::GLuint,
    pub width: f32,
    pub height: f32,
}

pub fn load(gl_context: &glutin::Context<PossiblyCurrent>) -> Gl {
    let gl =
        gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);

    let program = unsafe {
        let vs = gl.CreateShader(gl::VERTEX_SHADER);
        gl.ShaderSource(
            vs,
            1,
            [VS_SRC.as_ptr() as *const _].as_ptr(),
            std::ptr::null(),
        );
        gl.CompileShader(vs);

        let fs = gl.CreateShader(gl::FRAGMENT_SHADER);
        gl.ShaderSource(
            fs,
            1,
            [FS_SRC.as_ptr() as *const _].as_ptr(),
            std::ptr::null(),
        );
        gl.CompileShader(fs);

        let program = gl.CreateProgram();
        gl.AttachShader(program, vs);
        gl.AttachShader(program, fs);
        gl.LinkProgram(program);
        gl.UseProgram(program);

        let mut vb = std::mem::zeroed();
        gl.GenBuffers(1, &mut vb);
        gl.BindBuffer(gl::ARRAY_BUFFER, vb);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<f32>())
                as gl::types::GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        if gl.BindVertexArray.is_loaded() {
            let mut vao = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        let pos_attrib =
            gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
        let color_attrib =
            gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
        gl.VertexAttribPointer(
            pos_attrib as gl::types::GLuint,
            2,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl.VertexAttribPointer(
            color_attrib as gl::types::GLuint,
            3,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            (2 * std::mem::size_of::<f32>()) as *const () as *const _,
        );
        gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
        gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);
        program
    };

    Gl { gl: gl, program, width: 1600.0, height: 1200.0 }
}

impl Gl {
    pub fn draw_frame(&self) {
        unsafe {
            let width_location = self.gl.GetUniformLocation(self.program, CString::new("windowWidth").unwrap().as_ptr());
            let height_location = self.gl.GetUniformLocation(self.program, CString::new("windowHeight").unwrap().as_ptr());
            self.gl.Uniform1f(width_location, self.width);
            self.gl.Uniform1f(height_location, self.height);

            self.gl.ClearColor(0.8, 0.8, 0.8, 1.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width as f32;
        self.height = height as f32;
        println!("Resize {}x{}", width, height);
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
     0.0,   0.0,      0.0, 1.0,  0.0,
     800.0, 0.0,      0.0, 0.0,  1.0,
     0.0,   -800.0,   1.0, 0.0,  0.0,
];

const VS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;
uniform float windowWidth;
uniform float windowHeight;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position.x / windowWidth - 1.0, position.y / windowHeight + 1.0, 0.0, 1.0);
    v_color = color;
}
\0";

const FS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";

