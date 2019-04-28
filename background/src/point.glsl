uniform mat4 proj_view;
uniform float global_particle_scale;

attribute vec4 position_scale;
attribute vec4 color;

varying vec4 v_color;

void main() {
	gl_Position = proj_view * vec4(position_scale.xyz, 1.0);
	gl_PointSize = position_scale.w * global_particle_scale;
	v_color = color;
}


/* @@@ */


precision mediump float;

varying vec4 v_color;

void main() {
	vec2 uv = 2.0 * abs(vec2(0.5) - gl_PointCoord);
	float dist = length(uv);
	float fade = cos(min(pow(dist, 4.0), 1.0) * 3.14159 / 2.0);

	float alpha = v_color.a * fade;

	gl_FragColor = vec4(v_color.rgb * alpha, alpha);
}