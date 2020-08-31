#version 440 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec4 a_Color;
layout (location = 2) in float a_temp;

out vec4 v_Color;
out vec2 v_Position;

uniform vec2 camera_position;
uniform float scale;

uniform mat2 rotation;

vec4 black_body_emission(vec4 color){
  return color;
  vec3 red = vec3(1.0, 0.22, 0.0);
  vec3 white = vec3(1.0, 1.0, 1.0);
  vec3 blue = vec3(0.902, 1.0, 1.0);
}

void main()
{
    v_Position = Position;

    vec2 n_Position = (Position - camera_position) / scale;
    v_Color = a_Color;
    gl_Position = vec4(rotation * n_Position, 1.0, 1.0);
}
