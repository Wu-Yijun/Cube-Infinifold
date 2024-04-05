const int permutation[4] = int[4](1, 0, 3, 2);
const highp float zRatio = -0.1;
const highp float zoffset = 0.001;
const mat3x3 pj = mat3x3(0.866025, -0.5, zRatio,  // x -> x'
                         0.0, 1.0, zRatio,        // y -> y'
                         -0.866025, -0.5, zRatio  // z -> z'
);

out vec4 v_color;
out highp float mask_dist;

uniform bool u_base_layer;
uniform mat4 u_colors;
uniform mat4x3 u_points;
uniform mat3 u_proj;

uniform bool u_use_mask;
uniform vec3 u_mask_pos;
uniform vec3 u_mask_dir;
uniform float u_x_scale;

void main() {
  mat3 view = pj * u_proj;
  if (u_base_layer) {
    v_color = u_colors[gl_VertexID];
    gl_Position = vec4(view * u_points[gl_VertexID], 1.0);
  } else {
    v_color = u_colors[permutation[gl_VertexID]] * 0.5;
    gl_Position = vec4(view * u_points[permutation[gl_VertexID]], 1.0);
    gl_Position.z += zoffset;
  }
  if(u_use_mask){
    vec2 p = (view * u_mask_pos).xy;
    vec2 d = (view * u_mask_dir).xy;
    mask_dist = d.y * gl_Position.x - d.x * gl_Position.y + (p.y * d.y - p.x * d.x);
  }else{
    mask_dist = 1.0;
  }
  gl_Position.x *= u_x_scale;
}