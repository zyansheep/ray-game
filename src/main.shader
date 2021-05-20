#version 450
layout(location = 0) out vec4 o_Target;
/* layout(set = 2, binding = 0) uniform RayMaterial_color {
	vec4 color;
}; */
layout(set = 1, binding = 1) uniform RayUniform_camera_position {
	vec3 camera_position;
};
layout(set = 2, binding = 1) uniform RayUniform_model_translation {
	vec3 model_translation;
};
layout(set = 3, binding = 1) uniform RayUniform_light_translation {
	vec3 light_translation;
};
/* layout(set = 4, binding = 1) uniform RayUniform_time {
	float time;
}; */
layout(location = 1) in vec4 FragPos;

//Distance to scene at point
float mainSDF(vec3 p){
	return length(p - vec3(0.0)) - 0.5;
}

//Estimate normal based on mainSDF function
const float EPS=0.01;
const float ITER_MAX=30;
vec3 estimateNormal(vec3 p){
	float xPl = mainSDF(vec3(p.x+EPS, p.y, p.z));
	float xMi = mainSDF(vec3(p.x-EPS, p.y, p.z));
	float yPl = mainSDF(vec3(p.x, p.y+EPS, p.z));
	float yMi = mainSDF(vec3(p.x, p.y-EPS, p.z));
	float zPl = mainSDF(vec3(p.x, p.y, p.z+EPS));
	float zMi = mainSDF(vec3(p.x, p.y, p.z-EPS));
	float xDiff = xPl - xMi;
	float yDiff = yPl - yMi;
	float zDiff = zPl - zMi;
	return normalize(vec3(xDiff,yDiff,zDiff));
}
void main(){
	vec3 rayStart = FragPos.xyz - model_translation;
	// Ray Origin
	vec3 ro = rayStart;
	// Ray Direction
	vec3 rd = normalize(FragPos.xyz - camera_position);
	vec3 light_translation = light_translation - model_translation;

	//float totalDist = 0.0;
	float currentDist = mainSDF(ro);
	for(int iters=0; iters < ITER_MAX && currentDist > 0.01; iters++){
		ro += currentDist * rd;
		//totalDist += currentDist;
		currentDist = mainSDF(ro);
	}

	vec4 color = vec4(0.2, 0.2, 0.2, 0.0);
	if (currentDist < 0.01) {
		vec3 normal = estimateNormal(ro);
		float dotSN = dot(normal, normalize(light_translation - ro));
		color = vec4( (0.5 + 0.5 * normal) * dotSN,1.0);
	}
	o_Target = color;
}