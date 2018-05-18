#ifndef GL_ES
#define attribute in
#define varying out
#endif

#include "unrust/default_uniforms.glsl"

attribute vec3 aVertexPosition;
uniform mat4 uShadowMatrix;            

void main(void) {
    vec4 pos = uShadowMatrix * uMMatrix * vec4(aVertexPosition, 1.0);    
    pos.z *= pos.w;
    gl_Position = pos;
}