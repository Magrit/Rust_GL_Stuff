#version 450

layout(set = 0, binding = 0) uniform UBOData {
	mat4 viewMatrix;
} ubo_data;

layout(location = 0) in vec3 a_pos;

out gl_PerVertex {
	vec4 gl_Position;
};


void main() 
{
	gl_Position = vec4(a_pos, 1.0);
}