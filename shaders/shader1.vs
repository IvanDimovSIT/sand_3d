#version 330

layout(location = 0) in vec3 vertex;
layout(location = 1) in vec2 texcoord;
layout(location = 2) in vec3 normal;

out vec2 fragTexCoord;
out vec3 fragNormal;
out vec3 fragWorldPos;

uniform mat4 modelViewProjection;

void main()
{
    fragTexCoord = texcoord;
    fragNormal = mat3(transpose(inverse(modelViewProjection))) * normal;
    fragWorldPos = (modelViewProjection * vec4(vertex, 1.0)).xyz;
    gl_Position = modelViewProjection * vec4(vertex, 1.0);
}