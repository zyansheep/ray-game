#version 450
layout(location = 0) out vec4 o_Target;
/* layout(set = 2, binding = 0) uniform RayMaterial_color {
	vec4 color;
}; */
layout(set = 2, binding = 0) uniform CameraUniform_camera_position {
	vec3 camera_position;
};
layout(set = 3, binding = 0) uniform CameraUniform_camera_direction {
	vec3 camera_direction;
};
layout(location = 1) in vec4 FragPos;

struct ray {
	vec3 pos;
	vec3 dir;
};

//Distance to scene at point
float mainSDF(vec3 p){
	return length(p - vec3(0.0, 0.5, 0.0)) - 0.5;
}
//Estimate normal based on mainSDF function
const float EPS=0.01;
vec3 estimateNormal(vec3 p){
	float xPl=mainSDF(vec3(p.x+EPS,p.y,p.z));
	float xMi=mainSDF(vec3(p.x-EPS,p.y,p.z));
	float yPl=mainSDF(vec3(p.x,p.y+EPS,p.z));
	float yMi=mainSDF(vec3(p.x,p.y-EPS,p.z));
	float zPl=mainSDF(vec3(p.x,p.y,p.z+EPS));
	float zMi=mainSDF(vec3(p.x,p.y,p.z-EPS));
	float xDiff=xPl-xMi;
	float yDiff=yPl-yMi;
	float zDiff=zPl-zMi;
	return normalize(vec3(xDiff,yDiff,zDiff));
}
void main(){
	/* vec2 uv=gl_FragCoord.xy/iResolution.xy;
	uv-=vec2(0.5);//offset, so center of screen is origin
	uv.x*=iResolution.x/iResolution.y;//scale, so there is no rectangular distortion */
   
	vec3 camPos=camera_position;
	vec3 lookAt=camera_direction;
	//o_Target = gl_FragCoord;
	//o_Target = vec4(lookAt, 1);
	float zoom = 0.1;
	
	//ray camRay = create_camera_ray(FragPos.xyz, camPos, lookAt, zoom);
	/* ray camRay = ray(
		FragPos.xyz,

	) */
	ray camRay = ray(FragPos.xyz, normalize(FragPos.xyz - camPos));

	float totalDist=0.0;
	float finalDist=mainSDF(camRay.pos);
	
	int maxIters=30;
	for(int iters=0; iters < maxIters && finalDist > 0.001; iters++){
		camRay.pos+=finalDist*camRay.dir;
		totalDist+=finalDist;
		finalDist=mainSDF(camRay.pos);
	}
	vec4 color = vec4(0.0, 0.0, 0.0, 0.0);
	if (finalDist < 0.01) {
		vec3 normal = estimateNormal(camRay.pos);
		vec3 lightPos=vec3(2.0,1.0,1.0);
		float dotSN=dot(normal,normalize(lightPos-camRay.pos));
		color = vec4(0.5+0.5*normal,1.0) * dotSN; //estimateNormal(camRay.pos);
	}
	o_Target = color;
	
}