extern crate web_sys; 
extern crate js_sys;
extern crate wasm_bindgen;

use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};
use super::enums::{DataType};
use super::errors::*;
use super::renderer::*; 
use super::errors::*;

pub enum UniformData {
        FLOAT_1 (f32),
        FLOAT_VEC_1([f32;1]),
        INT_1 (u32),
        INT_VEC1 ([u32;1]),

        FLOAT_2(f32, f32),
        FLOAT_VEC_2([f32;2]),
        INT_2 (u32, u32),
        INT_VEC2 ([u32;2]),

        FLOAT_3(f32, f32, f32),
        FLOAT_VEC_3([f32;3]),
        INT_3 (u32, u32, u32),
        INT_VEC3 ([u32;3]),

        FLOAT_4(f32, f32, f32, f32),
        FLOAT_VEC_4([f32;4]),
        INT_4 (u32, u32, u32, u32),
        INT_VEC4 ([u32;4]),

        MATRIX_2FV([f32;4]),
        MATRIX_3FV([f32;9]),
        MATRIX_4FV([f32;16]),
}

pub fn get_uniform_location(gl:&WebGlRenderingContext, program:&WebGlProgram, name:&str) -> Result<WebGlUniformLocation, Error> {
    gl.get_uniform_location(&program, &name)
        .ok_or(Error::from(NativeError::UniformLocation))
}

pub fn set_uniform_data(gl:&WebGlRenderingContext, loc:&WebGlUniformLocation, transpose: bool, data: &UniformData) {
        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.uniform1f

        let loc = Some(loc);
        match *data {
                UniformData::FLOAT_1(a) => gl.uniform1f(loc, a),
                //TODO - the rest!
                _ => {} 
        }
}
/*

 uniform1f: _setSingleValue(loc => (v:GLfloat) => gl.uniform1f(loc, v)),
    uniform1fv: _setValues (UNIFORM_TYPE.FLOAT) (loc => (v:Float32List) => gl.uniform1fv(loc, v)),
    uniform1i: _setSingleValue(loc => (v:GLint) => gl.uniform1i(loc, v)),
    uniform1iv: _setValues (UNIFORM_TYPE.INT) (loc => (v:Int32List) => gl.uniform1iv(loc, v)),

    uniform2f: _setValues (UNIFORM_TYPE.FLOAT) (loc => (v:Float32List) => gl.uniform2f(loc, v[0], v[1])),
    uniform2fv: _setValues (UNIFORM_TYPE.FLOAT) (loc => (v:Float32List) => gl.uniform2fv(loc, v)),
    uniform2i: _setValues (UNIFORM_TYPE.INT) (loc => (v:Int32List) => gl.uniform2i(loc, v[0], v[1])),
    uniform2iv: _setValues (UNIFORM_TYPE.INT) (loc => (v:Int32List) => gl.uniform2iv(loc, v)),

    uniform3f: _setValues (UNIFORM_TYPE.FLOAT) (loc => (v:Float32List) => gl.uniform3f(loc, v[0], v[1], v[2])),
    uniform3fv: _setValues (UNIFORM_TYPE.FLOAT) (loc => (v:Float32List) => gl.uniform3fv(loc, v)),
    uniform3i: _setValues (UNIFORM_TYPE.INT) (loc => (v:Int32List) => gl.uniform3i(loc, v[0], v[1], v[2])),
    uniform3iv: _setValues (UNIFORM_TYPE.INT) (loc => (v:Int32List) => gl.uniform3iv(loc, v)),

    uniform4f: _setValues (UNIFORM_TYPE.FLOAT) (loc => (v:Float32List) => gl.uniform4f(loc, v[0], v[1], v[2], v[3])),
    uniform4fv: _setValues (UNIFORM_TYPE.FLOAT) (loc => (v:Float32List) => gl.uniform4fv(loc, v)),
    uniform4i: _setValues (UNIFORM_TYPE.INT) (loc => (v:Int32List) => gl.uniform4i(loc, v[0], v[1], v[2], v[3])),
    uniform4iv: _setValues (UNIFORM_TYPE.INT) (loc => (v:Int32List) => gl.uniform4iv(loc, v)),

    uniformMatrix2fv: _setMatrixValues (UNIFORM_TYPE.FLOAT) (loc => t => (v:Float32List) => gl.uniformMatrix2fv(loc, t, v)),
    uniformMatrix3fv: _setMatrixValues (UNIFORM_TYPE.FLOAT) (loc => t => (v:Float32List) => gl.uniformMatrix3fv(loc, t, v)),
    uniformMatrix4fv: _setMatrixValues (UNIFORM_TYPE.FLOAT) (loc => t => (v:Float32List) => gl.uniformMatrix4fv(loc, t, v)),
    */