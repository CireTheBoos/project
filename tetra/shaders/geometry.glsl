#version 460

//-------// IN //-------//

layout(triangles) in;

//-------// OUT //-------//

layout(triangle_strip, max_vertices = 3) out;

layout(location = 0) flat out vec4 outColor;

//////////////////////////////////////////////////////
// Functions (from chatGPT + redesigned)
//////////////////////////////////////////////////////

// simple hash function to generate pseudo-random values
float rand_from_uint(uint seed) {
    seed = (seed ^ 61u) ^ (seed >> 16u);
    seed = seed + (seed << 3u);
    seed = seed ^ (seed >> 4u);
    seed = seed * 0x27d4eb2du;
    seed = seed ^ (seed >> 15u);
    return float(seed) / float(0xffffffffu); // normalize to [0.;1.]
}

vec3 random_color(uint seed) {
    float r = rand_from_uint(seed);
    float g = rand_from_uint(seed + 1u);
    float b = rand_from_uint(seed + 2u);

    return vec3(r, g, b);
}

//////////////////////////////////////////////////////
// Main
//////////////////////////////////////////////////////

void main() {
    // extract
    vec4 a = gl_in[0].gl_Position;
    vec4 b = gl_in[1].gl_Position;
    vec4 c = gl_in[2].gl_Position;

    // compute color from primitive ID
    outColor = vec4(random_color(gl_PrimitiveIDIn), 1.);

    // emit
    gl_Position = a;
    EmitVertex();
    gl_Position = b;
    EmitVertex();
    gl_Position = c;
    EmitVertex();

    EndPrimitive();
}