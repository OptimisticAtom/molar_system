#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec4 a_Color;

out vec4 v_Color;
out vec2 v_Position;

uniform mat2 rotation;

void main()
{
    v_Position = Position;
    v_Color = a_Color;
    gl_Position = vec4(rotation * Position, 1.0, 1.0);
}
