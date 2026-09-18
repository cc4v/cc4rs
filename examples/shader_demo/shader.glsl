@module shader_demo

@vs vs
in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
@end

@fs fs
layout(binding=0) uniform fs_params {
    vec4 params;
};

out vec4 frag_color;

void main() {
    vec2 uv = gl_FragCoord.xy / params.yz;
    float wave = sin(uv.x * 18.0 + params.x * 2.0) * 0.08;
    vec3 color = 0.5 + 0.5 * cos(params.x + uv.xyx * 4.0 + vec3(0.0, 2.0, 4.0));
    frag_color = vec4(color + wave, 1.0);
}
@end

@program shader_demo vs fs