#version 440 core

in vec2 v_Position;
in vec4 v_Color;
/* out vec4 Color; */
precision mediump float;

uniform float scale;

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

void main() {
  vec2 p = (1 - v_Position) / scale;
  float brightness = smoothNoise(p);
  vec3 c = (v_Color.rgb + brightness / 4.0) / 1.25;
  gl_FragColor.rgb = c;
  gl_FragColor.a = 1.;
}
