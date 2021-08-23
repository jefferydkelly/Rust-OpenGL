#version 330 core
layout (location = 0) in vec4 info;

out vec2 TexCoords;

void main()
{
    TexCoords = info.zw;
    gl_Position = vec4(info.xy, 0.0, 1.0); 
}  