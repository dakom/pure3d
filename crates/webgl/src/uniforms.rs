extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};
use super::enums::{DataType};
use super::errors::*;
use super::renderer::*; 
use super::errors::*;

pub enum UniformData<'a> {
        FLOAT_VAL_1(f32),
        FLOAT_1(&'a [f32]),
        INT_VAL_1 (i32),
        INT_1(&'a [i32]),

        FLOAT_VAL_2(f32, f32),
        FLOAT_2(&'a [f32]),
        INT_VAL_2 (i32, i32),
        INT_2(&'a [i32]),

        FLOAT_VAL_3(f32, f32, f32),
        FLOAT_3(&'a [f32]),
        INT_VAL_3 (i32, i32, i32),
        INT_3(&'a [i32]),

        FLOAT_VAL_4(f32, f32, f32, f32),
        FLOAT_4(&'a [f32]),
        INT_VAL_4 (i32, i32, i32, i32),
        INT_4(&'a [i32]),
}

pub enum UniformMatrixData <'a> {
        FLOAT_2(&'a [f32]),
        FLOAT_3(&'a [f32]),
        FLOAT_4(&'a [f32]),

        FLOAT_2_TRANSPOSED(&'a [f32]),
        FLOAT_3_TRANSPOSED(&'a [f32]),
        FLOAT_4_TRANSPOSED(&'a [f32]),
}

pub fn get_uniform_location(gl:&WebGlRenderingContext, program:&WebGlProgram, name:&str) -> Result<WebGlUniformLocation, Error> {
    gl.get_uniform_location(&program, &name)
        .ok_or(Error::from(NativeError::UniformLocation))
}

pub fn set_uniform_data(gl:&WebGlRenderingContext, loc:&WebGlUniformLocation, data: UniformData) {
        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.uniform1f

        //the only because the gl.uniform calls require passing a mutable matrix
        //see https://github.com/rustwasm/wasm-bindgen/issues/1131
        //and https://github.com/rustwasm/wasm-bindgen/issues/1005
        let mut mutable_v:[i32;4] = [0, 0, 0, 0];

        let loc = Some(loc);
        match data {
                UniformData::FLOAT_VAL_1(a) => gl.uniform1f(loc, a),
                UniformData::FLOAT_1(v) => gl.uniform1fv_with_f32_array(loc, v),
                UniformData::INT_VAL_1(a) => gl.uniform1i(loc, a),
                UniformData::INT_1(v) => {
                        mutable_v[0] = v[0];
                        gl.uniform1iv_with_i32_array(loc, &mut mutable_v);
                }
                
                UniformData::FLOAT_VAL_2(a, b) => gl.uniform2f(loc, a, b),
                UniformData::FLOAT_2(v) => gl.uniform2fv_with_f32_array(loc, v),
                UniformData::INT_VAL_2(a, b) => gl.uniform2i(loc, a, b),
                UniformData::INT_2(v) => {
                        mutable_v[0] = v[0];
                        mutable_v[1] = v[1];
                        gl.uniform2iv_with_i32_array(loc, &mut mutable_v);
                }

                UniformData::FLOAT_VAL_3(a, b, c) => gl.uniform3f(loc, a, b, c),
                UniformData::FLOAT_3(v) => gl.uniform3fv_with_f32_array(loc, v),
                UniformData::INT_VAL_3(a, b, c) => gl.uniform3i(loc, a, b, c),
                UniformData::INT_3(v) => {
                        mutable_v[0] = v[0];
                        mutable_v[1] = v[1];
                        mutable_v[2] = v[2];
                        gl.uniform3iv_with_i32_array(loc, &mut mutable_v);
                }

                UniformData::FLOAT_VAL_4(a, b, c, d) => gl.uniform4f(loc, a, b, c, d),
                UniformData::FLOAT_4(v) => gl.uniform4fv_with_f32_array(loc, v),
                UniformData::INT_VAL_4(a, b, c, d) => gl.uniform4i(loc, a, b, c, d),
                UniformData::INT_4(v) => {
                        mutable_v[0] = v[0];
                        mutable_v[1] = v[1];
                        mutable_v[2] = v[2];
                        mutable_v[3] = v[3];
                        gl.uniform4iv_with_i32_array(loc, &mut mutable_v);
                }

                _ => {} 
        }
}
pub fn set_uniform_matrix_data(gl:&WebGlRenderingContext, loc:&WebGlUniformLocation, data: UniformMatrixData) {
        let loc = Some(loc);

        match &data {
                UniformMatrixData::FLOAT_2(v) => gl.uniform_matrix2fv_with_f32_array(loc, false, v),
                UniformMatrixData::FLOAT_3(v) => gl.uniform_matrix3fv_with_f32_array(loc, false, v),
                UniformMatrixData::FLOAT_4(v) => gl.uniform_matrix4fv_with_f32_array(loc, false, v),

                UniformMatrixData::FLOAT_2_TRANSPOSED(v) => gl.uniform_matrix2fv_with_f32_array(loc, true, v),
                UniformMatrixData::FLOAT_3_TRANSPOSED(v) => gl.uniform_matrix3fv_with_f32_array(loc, true, v),
                UniformMatrixData::FLOAT_4_TRANSPOSED(v) => gl.uniform_matrix4fv_with_f32_array(loc, true, v),
                _ => {} 
        }
}