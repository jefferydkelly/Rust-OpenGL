#version 330 core


struct Material {
	sampler2D diffuse;
	sampler2D specular;
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

in VS_OUT {
	vec3 FragPos;
	vec3 Normal;
	vec2 TexCoords;
	vec4 FragPosLightSpace;
} fs_in;

out vec4 FragColor;

uniform vec3 lightPos;
uniform vec3 viewPos;

uniform Material material;
uniform sampler2D shadowMap;
uniform DirectionalLight dirLight;

#define NR_POINTS_LIGHTS 10
uniform PointLight pointLights[NR_POINTS_LIGHTS];
uniform int numPointLights;

uniform Spotlight spotlight;

vec3 CalcDirectionalLight(DirectionalLight light, vec3 norm, vec3 viewDir, float shadow);
vec3 CalcPointLight(PointLight light, vec3 norm, vec3 FragPos, vec3 viewDir);
vec3 CalcSpotlight(Spotlight light, vec3 norm, vec3 fragPos, vec3 viewDir);
float ShadowCalculation(vec4 fragLightSpace);

float bias = 0.005;
void main() 
{
	vec3 color = texture(material.diffuse, fs_in.TexCoords).rgb;
    vec3 normal = normalize(fs_in.Normal);
    vec3 lightColor = vec3(0.3);
    // ambient
    vec3 ambient = 0.3 * color;
    // diffuse
    vec3 lightDir = normalize(lightPos - fs_in.FragPos);
    float diff = max(dot(lightDir, normal), 0.0);
    vec3 diffuse = diff * lightColor;
    // specular
    vec3 viewDir = normalize(viewPos - fs_in.FragPos);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = 0.0;
    vec3 halfwayDir = normalize(lightDir + viewDir);  
    spec = pow(max(dot(normal, halfwayDir), 0.0), 64.0);
    vec3 specular = spec * lightColor;    
    // calculate shadow
    float shadow = ShadowCalculation(fs_in.FragPosLightSpace);                      
    vec3 lighting = (ambient + (1.0 - shadow) * (diffuse + specular)) * color;    
    
    FragColor = vec4(lighting, 1.0);
	/*
	vec3 norm = normalize(fs_in.Normal);
	vec3 viewDir = normalize(viewPos - fs_in.FragPos);
	float shadow = 1.0;//1.0 - ShadowCalculation(fs_in.FragPosLightSpace);
	vec3 result = CalcDirectionalLight(dirLight, norm, viewDir, shadow);
	for (int i = 0; i < numPointLights; i++) {
		result += CalcPointLight(pointLights[i], norm, fs_in.FragPos, viewDir);
	}
	
	result += CalcSpotlight(spotlight, norm, fs_in.FragPos, viewDir);

	
	FragColor = vec4(result, 1.0);
	*/
}

vec3 CalcDirectionalLight(DirectionalLight light, vec3 norm, vec3 viewDir, float shadow) {
	vec4 tex_color = texture(material.diffuse, fs_in.TexCoords);
	vec3 tex =tex_color.xyz;

	vec3 lightDir = normalize(-light.direction);

	float diff = max(dot(norm, lightDir), 0.0);

	vec3 reflectDir = reflect(-lightDir, norm);

	float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
	
	vec3 ambient_color = light.ambient * tex;
	vec3 diffuse_color = light.diffuse * diff * tex;
	vec3 specular_color = light.specular * (spec * texture(material.specular, fs_in.TexCoords).xyz);

	return ambient_color + shadow * (diffuse_color + specular_color);
}


vec3 CalcPointLight(PointLight light, vec3 norm, vec3 fragPos, vec3 viewDir) {
	vec4 tex_color = texture(material.diffuse, fs_in.TexCoords);
	vec3 tex =tex_color.xyz;

	vec3 lightDir = normalize(light.position - fragPos);

	float diff = max(dot(norm, lightDir), 0.0);
	vec3 reflectDir = reflect(-lightDir, norm);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

	float distance = length(light.position - fragPos);
	float attenuation = light.constant + (light.linear * distance) + (light.quadratic * (distance * distance));

	vec3 ambient_color = light.ambient * tex;
	vec3 diffuse_color = light.diffuse * diff * tex;
	vec3 specular_color = light.specular * (spec * texture(material.specular, fs_in.TexCoords).xyz);

	return (ambient_color +  diffuse_color + specular_color) / attenuation;
} 


vec3 CalcSpotlight(Spotlight light, vec3 norm, vec3 fragPos, vec3 viewDir) {
	
	vec3 tex = texture(material.diffuse, fs_in.TexCoords).rgb;
	
	vec3 lightDir = normalize(light.position - fragPos);
	float diff = max(dot(norm, lightDir), 0.0);
	vec3 reflectDir = reflect(-lightDir, norm);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);

	float distance = length(light.position - fragPos);
	float attenuation = 1.0 / (light.constant + (light.linear * distance) + (light.quadratic * (distance * distance)));

	float theta = dot(lightDir, normalize(-light.direction));
	float epsilon = light.cutoff - light.outerCutoff;
	float intensity = clamp((theta - light.outerCutoff) / epsilon, 0.0, 1.0);
	

	vec3 ambient_color = light.ambient * tex;
	vec3 diffuse_color = light.diffuse * diff * tex;
	vec3 specular_color = light.specular * spec * texture(material.specular, fs_in.TexCoords).xyz;
	
	return (ambient_color + diffuse_color + specular_color) * attenuation * intensity;
	
}

float ShadowCalculation(vec4 fragLightSpace) {
	vec3 projCoords = fragLightSpace.xyz / fragLightSpace.w;
	projCoords = projCoords * 0.5 + 0.5;
	if (projCoords.z > 1.0) {
		return 0;
	}
	float closestDepth = texture(shadowMap, projCoords.xy).r;

	float currentDepth = projCoords.z;
	float shadow = currentDepth - bias > closestDepth ? 1.0 : 0.0;

	return shadow;
}
