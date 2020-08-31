#version 440 core

in vec2 g_Position;
in vec4 g_Color;
in vec2 vertex_position;
in vec2 w_Position;
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

void main() {
  vec2 p = w_Position / 100.0;
  vec2 d = (vertex_position - w_Position) * 2.0;
  float brightness = nestedNoise(p) * 0.2;
  float brightness2 = nestedNoise(d);
  vec3 c = (g_Color.rgb + brightness*brightness2) / 2.0;
  gl_FragColor.rgb = c;
  gl_FragColor.a = 1.;
}
