precision highp float;

uniform mat4 proj_view;

attribute vec3 position;
attribute vec4 color;

varying vec4 v_color;

void main() {
	gl_Position = proj_view * vec4(position, 1.0);
	v_color = color;
}


/* @@@ */


precision highp float;

varying vec4 v_color;

void main() {
	gl_FragColor = vec4(v_color.rgb * v_color.a, v_color.a);
}