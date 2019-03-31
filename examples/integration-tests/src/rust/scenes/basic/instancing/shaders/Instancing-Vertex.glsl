precision mediump float;

attribute vec2 a_vertex;

varying vec2 v_uv;

uniform mat4 u_modelViewProjection;
uniform mat4 u_size;

void main() {
    gl_Position = u_modelViewProjection * (u_size * vec4(a_vertex,0,1));
    v_uv = a_vertex;
}
