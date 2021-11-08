use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject};

pub struct Geometry {
    pub positions: Option<Vec<f32>>,
    pub indexes: Option<Vec<u16>>,
    pub normals: Option<Vec<f32>>,
    pub vao: Option<WebGlVertexArrayObject>,
}

impl Geometry {
    pub fn new() -> Geometry {
        Geometry {
            positions: None,
            indexes: None,
            normals: None,
            vao: None,
        }
    }

    pub fn set_positions(&mut self, positions: &Vec<f32>) {
        self.positions.replace(positions.to_vec());
    }
    pub fn set_normals(&mut self, normals: &Vec<f32>) {
        self.normals.replace(normals.to_vec());
    }
    pub fn set_indexes(&mut self, indexes: &Vec<u16>) {
        self.indexes.replace(indexes.to_vec());
    }

    pub fn get_vao(
        &mut self,
        gl: &WebGl2RenderingContext,
        program: &WebGlProgram,
    ) -> Result<&WebGlVertexArrayObject, JsValue> {
        if self.vao.is_none() {
            let ibo = create_ibo_vector(&gl, self.indexes.as_ref().unwrap())?;
            let position_vbo = create_vbo_vector(gl, self.positions.as_ref().unwrap())?;
            let normal_vbo = create_vbo_vector(gl, self.normals.as_ref().unwrap())?;

            // setup buffers and attributes to the VAO
            let vao = gl
                .create_vertex_array()
                .ok_or("Could not create vertex array object")?;
            gl.bind_vertex_array(Some(&vao));

            // bind buffer data
            gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ibo));
            
            // set attribute types
            let position_attribute_location = gl.get_attrib_location(&program, "position") as u32;
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_vbo));
            gl.enable_vertex_attrib_array(position_attribute_location);
            gl.vertex_attrib_pointer_with_i32(
                position_attribute_location,
                3,
                WebGl2RenderingContext::FLOAT,
                false,
                0,
                0,
            );
            
            let normal_attribute_location = gl.get_attrib_location(&program, "normal") as u32;
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&normal_vbo));
            gl.enable_vertex_attrib_array(normal_attribute_location);
            gl.vertex_attrib_pointer_with_i32(
                normal_attribute_location,
                3,
                WebGl2RenderingContext::FLOAT,
                false,
                0,
                0,
            );
            
            gl.bind_vertex_array(None);

            self.vao = Some(vao);
        }

        Ok(self.vao.as_ref().unwrap())
    }
}

pub fn create_vbo_vector(
    gl: &WebGl2RenderingContext,
    data: &Vec<f32>,
) -> Result<WebGlBuffer, String> {
    let vbo = gl.create_buffer().ok_or("Failed to create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
    unsafe {
        let f32_array = js_sys::Float32Array::view(&(*data));
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &f32_array,
            WebGl2RenderingContext::STATIC_DRAW,
        )
    }
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

    Ok(vbo)
}

pub fn create_ibo_vector(
    gl: &WebGl2RenderingContext,
    data: &Vec<u16>,
) -> Result<WebGlBuffer, String> {
    let ibo = gl.create_buffer().ok_or("Failed to create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ibo));
    unsafe {
        let ui16_array = js_sys::Uint16Array::view(data);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &ui16_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);

    Ok(ibo)
}
