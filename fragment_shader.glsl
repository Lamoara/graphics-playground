#version 330 core

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

uniform sampler2D texture1;
uniform vec3 cameraPos;
uniform vec4 fogColor;
uniform float minDistance;
uniform float maxDistance;

out vec4 FragColor;

void main() {
    float distance = length(FragPos - cameraPos);

    float fogFactor = clamp((distance - minDistance) / (maxDistance - minDistance), 0.0, 1.0);

    vec4 objectColor = texture(texture1, TexCoords);

    FragColor = mix(objectColor, fogColor, fogFactor);
}