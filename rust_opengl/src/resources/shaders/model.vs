#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coords;
//layout (location = 3) in vec3 offset;
layout (location = 3) in mat4 model;

out vec2 TexCoords;
out vec3 Normal;
out vec3 FragPos;

//uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
	TexCoords = tex_coords;
	Normal = mat3(transpose(inverse(model))) * normal;
	FragPos = vec3(model * vec4(position, 1.0));
	//FragPos = vec3(model * vec4(position + offset, 1.0));
	//gl_Position = projection * view * model * vec4(position + offset, 1.0);
	gl_Position = projection * view * model * vec4(position, 1.0);
}