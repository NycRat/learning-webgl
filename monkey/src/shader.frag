#version 300 es

precision highp float;

// uniform vec4 u_color;

// in vec4 v_color;
in vec3 v_normal;

uniform vec3 u_reverse_light_direction;

out vec4 out_color;

void main() {
  // out_color = v_color;
  // out_color = texture(u_texture, v_tex_coord);
  // out_color = vec4(out_color.yxxw);

  vec3 normal = normalize(v_normal);
  
  // compute the light by taking the dot product
  // of the normal to the light's reverse direction
  float light = dot(normal, u_reverse_light_direction);
  if (light < 0.0) {
    // light += 1.2;
  }
  // light = max(light, 0.0) + 0.2;
  
  // out_color = u_color;
  out_color = vec4(1.0, 0.3, 0.8, 1.0);
  
  // Lets multiply just the color portion (not the alpha)
  // by the light
  out_color.rgb *= light;
}
