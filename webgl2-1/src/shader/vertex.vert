#version 300 es

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec3 normal;
uniform mat4 pvmMatrix;
uniform mat4 invMatrix;
uniform vec3 LightDirection;
uniform vec3 EyeDirection;
uniform vec4 AmbientColor;
out vec4 vColor;

void main() {
    vec3 invLight  = normalize(invMatrix * vec4(LightDirection, 0.0)).xyz;
    vec3 invEye    = normalize(invMatrix * vec4(EyeDirection, 0.0)).xyz;
    vec3 halfLE    = normalize(invMatrix * vec4(EyeDirection, 0.0)).xyz;
    float diffuse  = clamp(dot(normal, invLight), 0.1, 1.0);
    float specular = pow(clamp(dot(normal, halfLE), 0.0, 1.0), 50.0);
    vec4 Light     = color * vec4(vec3(diffuse), 1.0) + vec4(vec3(specular), 1.0);
    vColor         = Light + AmbientColor;
    gl_Position =  pvmMatrix * vec4(position, 1.0);
}