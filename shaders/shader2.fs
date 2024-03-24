#version 330

in vec2 fragTexCoord;
in vec3 fragNormal;
in vec3 fragWorldPos;

out vec4 fragColor;

uniform sampler2D albedoMap;
uniform sampler2D normalMap;
uniform sampler2D metallicMap;
uniform sampler2D roughnessMap;

uniform vec3 lightPos;
uniform vec3 camPos;

void main()
{
    vec3 albedo = texture(albedoMap, fragTexCoord).rgb;
    vec3 normal = normalize(texture(normalMap, fragTexCoord).rgb * 2.0 - 1.0);
    float metallic = texture(metallicMap, fragTexCoord).r;
    float roughness = texture(roughnessMap, fragTexCoord).r;

    vec3 viewDir = normalize(camPos - fragWorldPos);
    vec3 lightDir = normalize(lightPos - fragWorldPos);
    vec3 halfwayDir = normalize(viewDir + lightDir);

    float NdotL = max(dot(fragNormal, lightDir), 0.0);
    float NdotH = max(dot(fragNormal, halfwayDir), 0.0);
    float VdotH = max(dot(viewDir, halfwayDir), 0.0);

    vec3 specular = mix(vec3(0.04), albedo, metallic);

    float D = DistributionGGX(normal, halfwayDir, roughness);
    float G = GeometrySmith(normal, viewDir, lightDir, roughness);
    vec3 F = FresnelSchlick(max(dot(halfwayDir, viewDir), 0.0), specular);

    vec3 kS = F;
    vec3 kD = vec3(1.0) - kS;
    kD *= 1.0 - metallic;

    vec3 numerator = D * G * F;
    float denominator = 4 * max(dot(fragNormal, viewDir), 0.0) * max(dot(fragNormal, lightDir), 0.0) + 0.001;
    vec3 specularContrib = numerator / denominator;

    vec3 diffuseContrib = (vec3(1.0) - F) * albedo / 3.14159;

    vec3 color = (kD * diffuseContrib + specularContrib) * NdotL;

    fragColor = vec4(color, 1.0);
}