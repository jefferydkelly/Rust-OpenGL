#version 330 core

uniform vec3 glowColor;
out vec4 color;

uniform sampler2D sprite;
in vec2 TexCoords;
void main() {
	vec4 spriteColor = texture(sprite, TexCoords);
	color = vec4(glowColor, spriteColor.a);
}