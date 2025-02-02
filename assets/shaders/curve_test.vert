#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 2) in float Vertex_Curve_Length;
layout(location = 0) out vec2 uv;
layout(location = 1) out float curve_len;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    uv = Vertex_Uv;
    curve_len = Vertex_Curve_Length;
}