#version 450 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec4 a_Color;
layout (location = 2) in float a_temp;
layout (location = 3) in vec4 gran;
layout (location = 4) in vec4 mat_settings;

out vec4 v_Color;
out vec2 v_Position;
out float v_min_grain;
out float v_max_grain;
out float v_min_crystal;
out float v_max_crystal;
out float v_state_of_matter;
out float v_grain_func;
out float v_crystal_func;
out float v_crystal_shape;

uniform vec2 camera_position;
uniform float scale;

uniform mat2 rotation;

vec3 black_body_emission(float temp){
  vec3 red = vec3(1.0, 0.22, 0.0);
  vec3 white = vec3(1.0, 1.0, 1.0);
  vec3 blue = vec3(0.902, 1.0, 1.0);
  if (temp < 1000.0) {
    return vec3(1.0);
  }
  else if (temp < 6500.0){
    float norm_temp = (temp - 1000) / 5500.0;
    vec3 color = mix(red, white, norm_temp);
    return color;
  }
  else if (temp < 12000.0){
    float norm_temp = (temp - 1000) / 5500.0;
    vec3 color = mix(white, blue, norm_temp);
    return color;
  }
  else{
    return blue;
  }
}

void main()
{
    v_Position = Position;
    v_min_grain = gran.r;
    v_max_grain = gran.g;
    v_min_crystal = gran.b;
    v_max_crystal = gran.a;
    v_state_of_matter = mat_settings.r;
    v_grain_func = mat_settings.g;
    v_crystal_func = mat_settings.b;
    v_crystal_shape = mat_settings.a;
    vec2 n_Position = (Position - camera_position) / scale;
    if (a_temp >= 1000.0){
    v_Color = vec4(mix(a_Color.rgb, black_body_emission(a_temp), 0.8), a_Color.a);
    }
    else{
      v_Color = a_Color;
    }
    gl_Position = vec4(rotation * n_Position, 1.0, 1.0);
}
