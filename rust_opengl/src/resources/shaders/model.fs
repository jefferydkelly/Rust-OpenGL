#version 330 core


struct Material {
	vec3 specular;
	float shininess;
};

struct DirectionalLight {
	vec3 direction;

	vec3 ambient;
	vec3 diffuse;
	vec3 specular;
};

struct PointLight {
	vec3 position;

	vec3 ambient;
	vec3 diffuse;
	vec3 specular;

	float constant;
	float linear;
	float quadratic;
};

struct Spotlight {
	vec3 position;
	vec3 direction;
	float cutoff;
	float outerCutoff;

	vec3 ambient;
	vec3 diffuse;
	vec3 specular;

	float constant;
	float linear;
	float quadratic;
};

in vec2 TexCoords;
in vec3 Normal;
in vec3 FragPos;

out vec4 FragColor;

uniform vec3 viewPos;
uniform sampler2D diffuseTex;

uniform Material material;
uniform DirectionalLight dirLight;

#define NR_POINTS_LIGHTS 4
uniform PointLight pointLights[NR_POINTS_LIGHTS];

uniform Spotlight spotlight;

vec3 CalcDirectionalLight(DirectionalLight light, vec3 norm, vec3 viewDir);
vec3 CalcPointLight(PointLight light, vec3 norm, vec3 FragPos, vec3 viewDir);
vec3 CalcSpotlight(Spotlight light, vec3 norm, vec3 fragPos, vec3 viewDir);

void main() 
{
	
	vec3 norm = normalize(Normal);
	vec3 viewDir = normalize(viewPos - FragPos);

	vec3 result = CalcDirectionalLight(dirLight, norm, viewDir);
	for (int i = 0; i < NR_POINTS_LIGHTS; i++) {
		result += CalcPointLight(pointLights[i], norm, FragPos, viewDir);
	}
	result += CalcSpotlight(spotlight, norm, FragPos, viewDir);

	FragColor = vec4(result, 1.0);
}

vec3 CalcDirectionalLight(DirectionalLight light, vec3 norm, vec3 viewDir) {
	vec4 tex_color = texture(diffuseTex, TexCoords);
	vec3 tex =tex_color.xyz;

	vec3 lightDir = normalize(-light.direction);

	float diff = max(dot(norm, lightDir), 0.0);

	vec3 reflectDir = reflect(-lightDir, norm);

	float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
	
	vec3 ambient_color = light.ambient * tex;
	vec3 diffuse_color = light.diffuse * diff * tex;
	vec3 specular_color = light.specular * (spec * material.specular);

	return ambient_color + diffuse_color + specular_color;
}


vec3 CalcPointLight(PointLight light, vec3 norm, vec3 fragPos, vec3 viewDir) {
	vec4 tex_color = texture(diffuseTex, TexCoords);
	vec3 tex =tex_color.xyz;

	vec3 lightDir = normalize(light.position - fragPos);

	float diff = max(dot(norm, lightDir), 0.0);
	vec3 reflectDir = reflect(-lightDir, norm);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

	float distance = length(light.position - fragPos);
	float attenuation = light.constant + (light.linear * distance) + (light.quadratic * (distance * distance));

	vec3 ambient_color = light.ambient * tex;
	vec3 diffuse_color = light.diffuse * diff * tex;
	vec3 specular_color = light.specular * spec * material.specular;

	return (ambient_color + diffuse_color + specular_color) / attenuation;
} 


vec3 CalcSpotlight(Spotlight light, vec3 norm, vec3 fragPos, vec3 viewDir) {
	
	vec4 tex_color = texture(diffuseTex, TexCoords);
	vec3 tex =tex_color.xyz;
	

	
	
	vec3 lightDir = normalize(light.position - fragPos);

	float theta = dot(lightDir, normalize(-light.direction));
	float epsilon = light.cutoff - light.outerCutoff;
	float intensity = clamp((theta - light.outerCutoff) / epsilon, 0.0, 1.0);
	

	
	float diff = max(dot(norm, lightDir), 0.0);
	vec3 reflectDir = reflect(-lightDir, norm);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

	float distance = length(light.position - fragPos);
	float attenuation = 1.0 / (light.constant + (light.linear * distance) + (light.quadratic * (distance * distance)));

	vec3 ambient_color = light.ambient * tex;
	vec3 diffuse_color = light.diffuse * diff * tex;
	vec3 specular_color = light.specular * spec * material.specular;
	
	return (ambient_color + diffuse_color + specular_color) * attenuation * intensity;
	
}
