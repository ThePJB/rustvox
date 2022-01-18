in vec4 vert_colour;
in float depth;

out vec4 frag_colour;

void main() {


    
    vec3 fogColour = vec3(0.7,0.7,0.4);

    float b = 0.003;
    float fogAmount = 1.0 - exp(-depth * b);
    // float fogAmount = depth;
    // float fogAmount = 0.5;




    frag_colour = vec4(mix(vert_colour.xyz, fogColour, fogAmount), vert_colour.w);
    // frag_colour = vert_colour;
}