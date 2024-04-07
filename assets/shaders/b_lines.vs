const highp float zRatio = -0.1;
const mat3x3 pj = mat3x3(0.866025, -0.5, zRatio,  // x -> x'
                         0.0, 1.0, zRatio,        // y -> y'
                         -0.866025, -0.5, zRatio  // z -> z'
);

out vec4 v_color;
out highp float mask_dist;

uniform mat3 u_proj;
uniform float u_x_scale;

uniform vec3 u_color1;
uniform vec3 u_color2;
uniform vec3 u_pos1;
uniform vec3 u_pos2;
uniform bool u_use_mask;
uniform vec3 u_mask_pos;
uniform vec3 u_mask_dir;

void main() {
  mat3 view = pj * u_proj;
  if (gl_VertexID == 0) {
    v_color = vec4(u_color1, 1.0);
    gl_Position = vec4(view * u_pos1, 1.0);
  } else {
    v_color = vec4(u_color2, 1.0);
    gl_Position = vec4(view * u_pos2, 1.0);
  }
  if (u_use_mask) {
    vec2 p = (view * u_mask_pos).xy;
    vec2 d = (view * u_mask_dir).xy;
    mask_dist =
        (gl_Position.x * d.y - gl_Position.y * d.x) - (p.x * d.y - p.y * d.x);
  } else {
    mask_dist = 1.0;
  }
  gl_Position.x *= u_x_scale;
}