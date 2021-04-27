#![allow(unused_imports, unused_import_braces, dead_code)]
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::mouse::MouseMotion, prelude::*, reflect::TypeUuid, render::{camera::Camera, pipeline::{PipelineDescriptor, RenderPipeline}, render_graph::{AssetRenderResourcesNode, RenderGraph, base}, renderer::RenderResources, shader::{ShaderStage, ShaderStages}}};

mod player;
use player::Player;
mod camera;
use camera::{CameraFocusEvent, CameraInterpolation, OrbitCamera, OrbitCameraInitialState, OrbitCameraPlugin};

fn main() {
	App::build()
		.insert_resource(Msaa { samples: 8 })
		.add_plugins(DefaultPlugins)
		.add_asset::<RayMaterial>()
		// Orbit Camera Plugin
		.add_plugin(OrbitCameraPlugin::new(
			Vec3::new(0.0, 1.0, 0.0),
			Vec3::new(-3.0, 5.0, 0.0),
		))
		.add_startup_system(setup.system())
		.add_system(player_movement.system())
    	.add_system_to_stage(CoreStage::Update, material_update.system())
		.run();
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
struct RayMaterial {
	pub camera_position: Vec3,
	pub camera_direction: Vec3,
}

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(set = 0, binding = 0) uniform CameraViewProj {
	mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
	mat4 Model;
};
void main() {
	gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = include_str!("main.shader");

/// set up a simple 3D scene
fn setup(
	mut commands: Commands,
	mut pipelines: ResMut<Assets<PipelineDescriptor>>,
	mut shaders: ResMut<Assets<Shader>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<RayMaterial>>,
	mut render_graph: ResMut<RenderGraph>,
	camera: Res<OrbitCameraInitialState>,
) {
	// Create a new shader pipeline
	let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
		vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
		fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
	}));

	// Add an AssetRenderResourcesNode to our Render Graph. This will bind RayMaterial resources to
	// our shader
	render_graph.add_system_node(
		"ray_material",
		AssetRenderResourcesNode::<RayMaterial>::new(true),
	);

	// Add a Render Graph edge connecting our new "ray_material" node to the main pass node. This
	// ensures "ray_material" runs before the main pass
	render_graph
		.add_node_edge("ray_material", base::node::MAIN_PASS)
		.unwrap();

	// Create a new material
	let ray_material = materials.add(RayMaterial {
		camera_position: camera.initial_position,
		camera_direction: (camera.initial_position - camera.initial_focus).normalize(),
	});

	// plane
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		//material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..Default::default()
	});
	// character
	commands.spawn_bundle(Player::bundle(
		"You",
		PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
				pipeline_handle,
			)]),
			transform: Transform::from_xyz(0.0, 0.5, 0.0),
			..Default::default()
		},
	)).insert(ray_material);
	// light
	commands.spawn_bundle(PointLightBundle {
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..Default::default()
	});
	// camera
	commands.spawn_bundle(PerspectiveCameraBundle {
		transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
		..Default::default()
	});
}

fn material_update(
	mut material: Query<&mut RayMaterial>,
	camera: Query<(&OrbitCamera, &Transform)>,
) {
	let mut ray_material = material.single_mut().unwrap();
	let (camera, camera_transform) = camera.single().unwrap();
	ray_material.camera_position = camera_transform.translation;
	ray_material.camera_direction = camera_transform.translation - camera.focus;
}

fn player_movement(
	mut focus_broadcast: EventWriter<CameraFocusEvent>,
	keyboard_input: Res<Input<KeyCode>>,
	mut player_transform: Query<&mut Transform, With<Player>>,
) {
	let mut player_transform = player_transform.single_mut().unwrap();

	let mut moved = false;
	if keyboard_input.pressed(KeyCode::W) {
		//info!("'W' currently pressed");
		moved = true;
		player_transform.translation.x += 0.1;
	}
	if keyboard_input.pressed(KeyCode::S) {
		//info!("'S' currently pressed");
		moved = true;
		player_transform.translation.x -= 0.1;
	}
	if keyboard_input.pressed(KeyCode::A) {
		//info!("'A' currently pressed");
		moved = true;
		player_transform.translation.z -= 0.1;
	}
	if keyboard_input.pressed(KeyCode::D) {
		//info!("'A' currently pressed");
		moved = true;
		player_transform.translation.z += 0.1;
	}
	if moved {
		focus_broadcast.send(CameraFocusEvent::new(
			player_transform.translation,
			CameraInterpolation::None,
		));
	}
}