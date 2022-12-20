#version 330 core

in vec3 position;
in vec3 normal;
in vec2 texCoords;

out vec4 color;

uniform sampler2D grassTexture;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    // Berechne die Weltkoordinaten des Grasblatts
    vec3 worldPosition = (model * vec4(position, 1.0)).xyz;

    // Berechne die Kamera-Koordinaten des Grasblatts
    vec3 viewPosition = (view * vec4(worldPosition, 1.0)).xyz;

    // Berechne die Projektions-Koordinaten des Grasblatts
    vec4 clipPosition = projection * vec4(viewPosition, 1.0);

    // Berechne die Textur-Koordinaten für das Grasblatt
    vec2 texCoords = position.xy * 0.1 + sin(position.x * 10.0) * 0.1;

    // Berechne die Farbe des Grasblatts anhand der Textur-Koordinaten
    vec4 grassColor = texture(grassTexture, texCoords);

    // Berechne die endgültige Farbe des Grasblatts
    color = grassColor;
}
