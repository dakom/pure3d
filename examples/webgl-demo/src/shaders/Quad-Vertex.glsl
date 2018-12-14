attribute vec2 a_Vertex;

uniform vec4 u_Color;
uniform mat4 u_Transform;
uniform mat4 u_Size;
    
void main() {     
    gl_Position = u_Transform * (u_Size * vec4(a_Vertex,0,1));
}
