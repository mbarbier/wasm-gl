#version 300 es

in vec3 position;
in vec3 normal;
in vec4 color;

uniform mat4 modelMatrix;
uniform mat4 modelViewMatrix;
uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform vec3 cameraPosition;
uniform vec3 lightDirection;
uniform vec4 ambientColor;

out vec3 vNormal;
out vec4 vColor;
out vec3 vPosition;

void main(void) {
    vPosition = (modelViewMatrix * vec4(position, 1.0)).xyz;
    vColor = vec4(0.8, 0.0, 0.2, 1.0);

    mat3 normalMatrix = transpose(inverse(mat3(modelViewMatrix)));
    vNormal = normalize(normalMatrix * normal);
    // vNormal = normalize(normal);

    gl_Position =  projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
