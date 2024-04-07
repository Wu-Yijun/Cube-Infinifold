in vec4 v_color;
in highp float mask_dist;
out vec4 out_color;
void main() {
  if (mask_dist < 0.0) {
    discard;
  }
  out_color = v_color;
}