use bevy::math::Vec3;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;

#[derive(RenderResources, Default, TypeUuid)]
#[repr(C)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct RayUniform {
	pub camera_position: Vec3,
	pub model_translation: Vec3,
	pub light_translation: Vec3,
	pub time: f32,
}
