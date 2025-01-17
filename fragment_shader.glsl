#version 330 core

in vec3 FragPos;

uniform vec3 camPos;

out vec4 FragColor;

void main() {
    float distance = length(FragPos - camPos);
    FragColor = vec4(1.0, 1.0, 1.0, 1.0) - vec4(1.0, 1.0, 1.0, 0.0) * distance * distance * 0.1;
}