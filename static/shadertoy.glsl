#ifndef GL_ES
#define varying in
#define gl_FragColor FragColor
#define texture2D texture
out vec4 FragColor;
#endif

varying vec2 vTextureCoord;

// shadertoy
uniform float iTime;        // Current time in seconds
uniform float iTimeDelta;   // Time it takes to render a frame, in seconds
uniform vec3 iResolution;   // The viewport resolution (z is pixel aspect ratio, usually 1.0)
uniform int iFrame;         // Current frame

void mainImage( out vec4 fragColor, in vec2 fragCoord );

void main(void) {
    vec2 fragCoord = vTextureCoord.xy * iResolution.xy;
    vec4 fragColor;
    mainImage(fragColor, fragCoord);
    gl_FragColor = fragColor;
}
