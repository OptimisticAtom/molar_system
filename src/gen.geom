#version 450 core

layout (points) in;
layout (triangle_strip, max_vertices = 24) out;

in vec4 v_Color[];
in vec2 v_Position[];
in float v_min_grain[];
in float v_max_grain[];
in float v_min_crystal[];
in float v_max_crystal[];
in float v_state_of_matter[];
in float v_grain_func[];
in float v_crystal_func[];
in float v_crystal_shape[];


out vec4 g_Color;
out vec2 g_Position;
out vec2 vertex_position;
out vec2 w_Position;
out float g_min_grain;
out float g_max_grain;
out float g_min_crystal;
out float g_max_crystal;
out float g_state_of_matter;
out float g_grain_func;
out float g_crystal_func;
out float g_crystal_shape;


uniform vec2 camera_position;
uniform float scale;
uniform mat2 rotation;

void create_vertex(vec2 position){
  g_Position = position;
  gl_Position = vec4(g_Position, 1.0, 1.0);
  g_Color = v_Color[0];
  EmitVertex();
}


void main(){
  // float x = gl_in[0].gl_Position.x - camera_position.x;
  // float y = gl_in[0].gl_Position.y - camera_position.y;
  vertex_position = v_Position[0];
  g_max_grain = v_max_grain[0];
  g_state_of_matter = v_state_of_matter[0];
  float x = v_Position[0].x;
  float y = v_Position[0].y;
  vec2 center_position = rotation * ((v_Position[0] - camera_position) / scale);
  float distance_x = 0.5;
  float distance_y = 0.2886751346;
  vec2 world_positions[] = vec2[6](
      vec2(x, (y + (0.5773502692))),
      vec2((x + distance_x), (y + distance_y)),
      vec2((x + distance_x), (y - distance_y)),
      vec2(x, (y - (0.5773502692))),
      vec2((x - distance_x), (y - distance_y)),
      vec2((x - distance_x), (y + distance_y))
    );
  vec2 positions[] = vec2[6](
            rotation * ((world_positions[0] - camera_position) / scale),
            rotation * ((world_positions[1] - camera_position) / scale),
            rotation * ((world_positions[2] - camera_position) / scale),
            rotation * ((world_positions[3] - camera_position) / scale),
            rotation * ((world_positions[4] - camera_position) / scale),
            rotation * ((world_positions[5] - camera_position) / scale)
        );
  for(int i = 0; i < 6; i++)
  {
    for(int j = 0; j < 3; j++)
    {
      switch(j)
      {
        case 0:
          w_Position = v_Position[0];
          create_vertex(center_position);
          break;

        case 1:
          w_Position = world_positions[i];
          create_vertex(positions[i]);
          break;

        case 2:
          if(i == 5)
          {
            w_Position = world_positions[0];
            create_vertex(positions[0]);
          }
          else{
            w_Position = world_positions[i + 1];
            create_vertex(positions[i + 1]);
          }
          break;

        default:
          break;
      }
    }
    EndPrimitive();
  }

  // vec2 position = vec2(x, (y + (0.5773502692)));
  // g_Position = (rotation * position) / scale;
  // gl_Position = vec4(g_Position, 0.0, 0.0);
  // g_Color = v_Color[0];
  // EmitVertex();
  //
  // position = vec2((x + distance_x), (y + distance_y));
  // g_Position = (rotation * position) / scale;
  // gl_Position = vec4(g_Position, 0.0, 0.0);
  // g_Color = v_Color[0];
  // EmitVertex();
  //
  // position = vec2((x + distance_x), (y - distance_y));
  // g_Position = (rotation * position) / scale;
  // gl_Position = vec4(g_Position, 0.0, 0.0);
  // g_Color = v_Color[0];
  // EmitVertex();
  //
  // position = vec2(x, (y - (0.5773502692)));
  // g_Position = (rotation * position) / scale;
  // gl_Position = vec4(g_Position, 0.0, 0.0);
  // g_Color = v_Color[0];
  // EmitVertex();
  //
  // position = vec2((x - distance_x), (y - distance_y));
  // g_Position = (rotation * position) / scale;
  // gl_Position = vec4(g_Position, 0.0, 0.0);
  // g_Color = v_Color[0];
  // EmitVertex();
  //
  // position = vec2((x - distance_x), (y + distance_y));
  // g_Position = (rotation * position) / scale;
  // gl_Position = vec4(g_Position, 0.0, 0.0);
  // g_Color = v_Color[0];
  // EmitVertex();

}
