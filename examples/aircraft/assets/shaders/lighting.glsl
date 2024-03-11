struct DirLight {
    vec3 direction;
    
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position_low;
    vec3 position_high;
    
    float constant;
    float linear;
    float quadratic;
    
    vec3 diffuse;
    vec3 specular;   
};

struct SpotLight {
    vec3 position_low;
    vec3 position_high;
    vec3 direction;

    float cutOff;
    float outerCutOff;
    float constant;
    float linear;
    float quadratic;

    vec3 diffuse;
    vec3 specular;
};