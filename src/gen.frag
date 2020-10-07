#version 450 core

in vec2 g_Position;
in vec4 g_Color;
in vec2 vertex_position;
in vec2 w_Position;
in float g_max_grain;
flat in float g_state_of_matter;
/* out vec4 Color; */
precision mediump float;

uniform float scale;
uniform float time;

float random(float p) {
  return fract(sin(p)*10000.);
}

float noise(vec2 p) {
  return random(p.x + p.y*10000.);
}

vec2 sw(vec2 p) {return vec2( floor(p.x) , floor(p.y) );}
vec2 se(vec2 p) {return vec2( ceil(p.x)  , floor(p.y) );}
vec2 nw(vec2 p) {return vec2( floor(p.x) , ceil(p.y)  );}
vec2 ne(vec2 p) {return vec2( ceil(p.x)  , ceil(p.y)  );}

float smoothNoise(vec2 p) {
  vec2 inter = smoothstep(0., 1., fract(p));
  float s = mix(noise(sw(p)), noise(se(p)), inter.x);
  float n = mix(noise(nw(p)), noise(ne(p)), inter.x);
  return mix(s, n, inter.y);
  return noise(nw(p));
}

float movingNoise(vec2 p) {
  float total = 0.0;
  total += smoothNoise(p - time);
  total += smoothNoise(p*2. + time) / 2.;
  total += smoothNoise(p*4. - time) / 4.;
  total += smoothNoise(p*8. + time) / 8.;
  total += smoothNoise(p*16.- time) / 16.;
  total /= 1. + 1./2. + 1./4. + 1./8. + 1./16.;
  return total;
}

float nestedNoise(vec2 p) {
  float x = movingNoise(p);
  float y = movingNoise(p + 100.);
  return movingNoise(p + vec2(x, y));
}

float Feather(vec2 p){
  float d = length(p);
  return d;
}

vec4 Solid(){
  vec2 p = w_Position;
  vec2 d = (vertex_position - w_Position) * 2.0;
  float brightness = nestedNoise(p);
  float id_scale = 10.;
  if(g_max_grain > 0)
  {
    id_scale = .5 / g_max_grain;
  }
  vec2 id = floor(d*id_scale);
  vec2 gid = floor(p*id_scale);
  float center_id = clamp(Feather(id), 0.0, id_scale);
  float brightness2 = noise(vec2(1.)*sin(gid));
  vec3 c = (mix(g_Color.rgb, vec3(fract(brightness2)), .1));
  return vec4(c, 1.);
}

vec4 Liquid(){
  vec2 p = w_Position;
  vec2 d = (vertex_position - w_Position) * 2.0;
  float brightness = nestedNoise(mod(p, 30));
  vec3 c = mix(g_Color.rgb, vec3(fract(brightness)), 0.3);
  return vec4(c, g_Color.a);
}

vec4 Gas(){
  vec2 p = w_Position;
  vec2 d = (vertex_position - w_Position) * 2.0;
  float brightness = nestedNoise(mod(p, 30));
  vec3 c = mix(g_Color.rgb, vec3(fract(brightness)), 0.06);
  return vec4(c, g_Color.a);
}

void main() {
  // vec4 c = vec4(1.);
  vec4 c = g_Color;
  int som = int(round(g_state_of_matter));
  switch(som)
  {
    case 0:
      c = Solid();
      break;

    case 254:
      c = Liquid();
      break;

    case 255:
      c = Gas();
      break;

    default:
      break;
  }
  // if(som == 0.)
  // {
  //   c = Solid();
  // }
  // else if(som == 254.)
  // {
  //   c = Liquid();
  // }
  // else{
  //
  // }
  gl_FragColor = c;
}
