#![allow(unused_imports, unused_import_braces, dead_code)]
use bevy::{
	diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
	input::mouse::MouseMotion,
	prelude::*,
	reflect::TypeUuid,
	render::{
		camera::Camera,
		pipeline::{PipelineDescriptor, RenderPipeline},
		render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
		renderer::RenderResources,
		shader::{ShaderStage, ShaderStages},
	},
	wgpu::WgpuOptions,
};
use heron::prelude::*;

mod player;
use player::Player;
mod camera;
use camera::{
	CameraFocusEvent, CameraInterpolation, OrbitCamera, OrbitCameraInitialState, OrbitCameraPlugin,
};
mod ray;
use ray::RayUniform;

fn main() {
	App::build()
		//.insert_resource(Msaa { samples: 8 })
		.insert_resource(WindowDescriptor {
			title: "Ray Game".to_string(),
			width: 1280.,
			height: 720.,
			vsync: true,
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(PhysicsPlugin::default())
		.insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
		// Orbit Camera Plugin
		.add_plugin(OrbitCameraPlugin::new(
			Vec3::new(0.0, 1.0, 0.0),
			Vec3::new(-3.0, 5.0, 0.0),
		))
		.add_startup_system(setup.system())
		.add_physics_system(player_movement.system())
		.add_system_to_stage(CoreStage::PostUpdate, uniform_update.system()) // Update uniforms after logic
		.run();
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
layout(location = 1) out vec4 FragPos;
void main() {
	FragPos = Model * vec4(Vertex_Position, 1.0);
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
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut render_graph: ResMut<RenderGraph>,
) {
	// Create a new shader pipeline
	let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
		vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
		fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
	}));

	render_graph.add_system_node("ray_uniform", RenderResourcesNode::<RayUniform>::new(true));
	// Add a `RenderGraph` edge connecting our new "ray_uniform" node to the main pass node. This
	// ensures that "ray_uniform" runs before the main pass.
	render_graph
		.add_node_edge("ray_uniform", base::node::MAIN_PASS)
		.unwrap();

	// plane with PBR material
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
			material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
			..Default::default()
		})
		.insert(Body::Cuboid {
			half_extends: Vec3::new(10.0, 0.05, 10.0),
		})
		.insert(BodyType::Static);
	// Character to move
	commands
		.spawn_bundle(Player::bundle(
			"You",
			PbrBundle {
				mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
				render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
					pipeline_handle.clone_weak(),
				)]),
				visible: Visible {
					is_transparent: true,
					..Default::default()
				},
				transform: Transform::from_xyz(0.0, 1.0, 0.0),
				..Default::default()
			},
		))
		.insert(RayUniform::default())
		.insert(Body::Cuboid {
			half_extends: Vec3::new(0.5, 0.5, 0.5),
		})
		.insert(BodyType::Kinematic); // Camera Position & Model translation Uniform

	// Other Box to push around
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
				pipeline_handle,
			)]),
			visible: Visible {
				is_transparent: true,
				..Default::default()
			},
			transform: Transform::from_xyz(0.0, 5.0, 3.0),
			..Default::default()
		})
		.insert(RayUniform::default())
		.insert(Body::Cuboid {
			half_extends: Vec3::new(0.5, 0.5, 0.5),
		})
		.insert(BodyType::Dynamic)
		.insert(PhysicMaterial {
			restitution: 0.5,
			density: 2.0,
			friction: 2.0,
		}); // Camera Position & Model translation Uniform

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

fn uniform_update(
	mut ray_uniforms: Query<(&Transform, &mut RayUniform)>,
	camera_transform: Query<&Transform, With<OrbitCamera>>,
	light_transform: Query<&Transform, With<PointLight>>,
	time: Res<Time>,
) {
	let camera_transform = camera_transform.single().unwrap();
	let light_transform = light_transform.single().unwrap();
	// Update all camera uniforms
	for (model_transform, mut ray_uniform) in ray_uniforms.iter_mut() {
		ray_uniform.camera_position = camera_transform.translation;
		ray_uniform.model_translation = model_transform.translation;
		ray_uniform.light_translation = light_transform.translation;
		ray_uniform.time = time.seconds_since_startup() as f32;
	}
}

fn player_movement(
	mut focus_broadcast: EventWriter<CameraFocusEvent>,
	keyboard_input: Res<Input<KeyCode>>,
	mut q: QuerySet<(
		Query<(&mut Transform, &Player)>,
		Query<&Transform, With<OrbitCamera>>,
	)>,
	/* mut player: Query<(&mut Transform, &Player), With<Player>>,
	camera_transform: Query<&Transform, With<OrbitCamera>>, */
) {
	let camera_transform = q.q1().single().unwrap();

	let mut moved = false;
	let camera_forward = camera_transform.forward();
	let camera_right = camera_transform.right();
	let forward_direction = Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize();
	let right_direction = Vec3::new(camera_right.x, 0.0, camera_right.z).normalize();

	let (mut player_transform, player) = q.q0_mut().single_mut().unwrap();
	if keyboard_input.pressed(KeyCode::W) {
		//info!("'W' currently pressed");
		moved = true;
		player_transform.translation += player.speed * forward_direction;
	}
	if keyboard_input.pressed(KeyCode::S) {
		//info!("'S' currently pressed");
		moved = true;
		player_transform.translation -= player.speed * forward_direction;
	}
	if keyboard_input.pressed(KeyCode::A) {
		moved = true;
		player_transform.translation -= player.speed * right_direction;
	}
	if keyboard_input.pressed(KeyCode::D) {
		moved = true;
		player_transform.translation += player.speed * right_direction;
	}
	// Up and Down
	if keyboard_input.pressed(KeyCode::Space) {
		moved = true;
		player_transform.translation.y += player.speed;
	}
	if keyboard_input.pressed(KeyCode::LShift) {
		moved = true;
		player_transform.translation.y -= player.speed;
	}
	if moved {
		focus_broadcast.send(CameraFocusEvent::new(
			player_transform.translation,
			CameraInterpolation::None,
		));
	}
}
