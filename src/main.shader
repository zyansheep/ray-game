#version 450
layout(location = 0) out vec4 o_Target;
/* layout(set = 2, binding = 0) uniform RayMaterial_color {
	vec4 color;
}; */
layout(set = 2, binding = 0) uniform RayMaterial_camera_position {
	vec3 camera_position;
};
layout(set = 3, binding = 0) uniform RayMaterial_camera_direction {
	vec3 camera_direction;
};
struct ray {
	vec3 pos;
	vec3 dir;
};
//Create the camera ray
ray create_camera_ray(vec2 uv, vec3 camPos, vec3 lookAt, float zoom){
	vec3 f = normalize(lookAt - camPos);
	vec3 r = cross(vec3(0.0,1.0,0.0),f);
	vec3 u = cross(f,r);
	vec3 c=camPos+f*zoom;
	vec3 i=c+uv.x*r+uv.y*u;
	vec3 dir=i-camPos;
	return ray(camPos,normalize(dir));
}
//Distance to scene at point
float distToScene(vec3 p){
	return length(p - vec3(0.0)) - 30;
}
//Estimate normal based on distToScene function
const float EPS=0.001;
vec3 estimateNormal(vec3 p){
	float xPl=distToScene(vec3(p.x+EPS,p.y,p.z));
	float xMi=distToScene(vec3(p.x-EPS,p.y,p.z));
	float yPl=distToScene(vec3(p.x,p.y+EPS,p.z));
	float yMi=distToScene(vec3(p.x,p.y-EPS,p.z));
	float zPl=distToScene(vec3(p.x,p.y,p.z+EPS));
	float zMi=distToScene(vec3(p.x,p.y,p.z-EPS));
	float xDiff=xPl-xMi;
	float yDiff=yPl-yMi;
	float zDiff=zPl-zMi;
	return normalize(vec3(xDiff,yDiff,zDiff));
}
void main(){
	vec2 iResolution = vec2(100.0,100.0);
	vec2 uv=gl_FragCoord.xy/iResolution.xy;
	uv-=vec2(0.5);//offset, so center of screen is origin
	uv.x*=iResolution.x/iResolution.y;//scale, so there is no rectangular distortion
   
	vec3 camPos=camera_position;
	vec3 lookAt=camera_direction;
	//o_Target = gl_FragCoord;
	o_Target = vec4(lookAt, 1);
	/* float zoom=0.1;
	
	ray camRay=create_camera_ray(uv,camPos,lookAt,zoom);
	
	float totalDist=0.0;
	float finalDist=distToScene(camRay.pos);
	int iters=0;
	int maxIters=20;
	for(iters=0;iters<maxIters&&finalDist>0.01;iters++){
		camRay.pos+=finalDist*camRay.dir;
		totalDist+=finalDist;
		finalDist=distToScene(camRay.pos);
	}
	vec3 normal=estimateNormal(camRay.pos);
	
	vec3 lightPos=vec3(2.0,1.0,1.0);
	
	float dotSN=dot(normal,normalize(lightPos-camRay.pos));
	
	o_Target=vec4(0.5+0.5*normal,1.0)*dotSN; */
}