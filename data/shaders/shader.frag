#version 330 core

in vec3 passed_Color;
out vec4 Color;

void main()
{
    Color = vec4(passed_Color, 1.0);
}
