precision mediump float;

attribute vec2 a_vertex;

uniform vec4 u_color;
uniform mat4 u_transform;
uniform mat4 u_size;

void main() {
    //gl_Position = u_transform * (u_size * vec4(a_vertex,0,1));
    gl_Position = vec4(a_vertex, 0.0, 1.0);
}
