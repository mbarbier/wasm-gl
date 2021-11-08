#version 300 es

precision highp float;

uniform mat4 modelMatrix;

uniform vec3 lightDirection;
uniform vec3 cameraPosition;
uniform vec4 ambientColor;

in  vec3 vPosition;
in  vec3 vNormal;
in  vec4 vColor;

out vec4 FragColor;

void main(void) {
    float ambientStrength = 0.05;
    vec3 lightColor = ambientColor.xyz;

    vec3 diffuse   = clamp(dot(vNormal, lightDirection), 0.0, 1.0) * lightColor;
    vec4 destColor = vColor * vec4(diffuse, 1.0) + ambientColor * ambientStrength;

    FragColor = destColor;
}
