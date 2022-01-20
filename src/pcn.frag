in vec4 vert_colour;
in float depth;

out vec4 frag_colour;

uniform float fog_intensity;
uniform vec3 fog_colour;

void main() {


    

    //float b = 0.003;
    float fogAmount = 1.0 - exp(-depth * fog_intensity);
    // float fogAmount = depth;
    // float fogAmount = 0.5;




    frag_colour = vec4(mix(vert_colour.xyz, fog_colour, fogAmount), vert_colour.w);
    // frag_colour = vert_colour;
}