use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

// pub let Empty : &str = "";

static EMPTY: &'static str = "";

pub struct Material<'a> {
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,

    program: Option<WebGlProgram>,
}

impl<'a> Material<'a> {
    pub fn new() -> Material<'a> {
        Material {
            program: None,
            vertex_shader: EMPTY,
            fragment_shader: EMPTY,
        }
    }

    pub fn get_program(&mut self, gl: &WebGl2RenderingContext) -> Result<&WebGlProgram, JsValue> {
        if self.program.is_none() {
            let vert_shader = compile_shader(
                gl,
                WebGl2RenderingContext::VERTEX_SHADER,
                self.vertex_shader,
            )?;
            let frag_shader = compile_shader(
                gl,
                WebGl2RenderingContext::FRAGMENT_SHADER,
                self.fragment_shader,
            )?;

            let program = link_program(gl, &vert_shader, &frag_shader)?;
            self.program = Some(program);
        }

        let a = self.program.as_ref().unwrap();
        Ok(a)
    }
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
