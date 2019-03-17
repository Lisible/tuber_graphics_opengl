#version 330 core

in vec3 passed_Color;
in vec2 passed_TextureCoordinates;

out vec4 Color;

uniform sampler2D ourTexture;

void main()
{
    Color = texture(ourTexture, passed_TextureCoordinates);
}
