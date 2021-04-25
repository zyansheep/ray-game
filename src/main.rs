#![allow(unused_imports, unused_import_braces, dead_code)]
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::mouse::MouseMotion, prelude::*, render::camera::Camera};

mod player;
use player::{Player};
mod camera;
use camera::{OrbitCameraPlugin, CameraFocusEvent, CameraInterpolation};

fn main() {
	App::build()
	.insert_resource(Msaa { samples: 8 })
	.add_plugins(DefaultPlugins)
	.add_plugin(OrbitCameraPlugin::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(-3.0, 5.0, 0.0)))
	.add_startup_system(setup.system())
	.add_system(player_movement.system())
	//.add_system(camera_movement_system.system())
	.run();
}

/// set up a simple 3D scene
fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// plane
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..Default::default()
	});
	// character
	commands.spawn_bundle(
		Player::bundle("You", PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
			material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
			transform: Transform::from_xyz(0.0, 0.5, 0.0),
			..Default::default()
		})
	);
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

fn player_movement(
	mut focus_broadcast: EventWriter<CameraFocusEvent>,
	keyboard_input: Res<Input<KeyCode>>,
	mut player_transform: Query<&mut Transform, With<Player>>
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
		focus_broadcast.send(CameraFocusEvent::new(player_transform.translation,CameraInterpolation::None) );
	}
}

/* fn camera_movement_system(
	mut mouse_motion_events: EventReader<MouseMotion>,
	mut q: QuerySet<(
		Query<&Transform, With<Player>>,
		Query<&mut Transform, With<Camera>>
	)>
) {
	let player_transform = q.q0().single().unwrap().clone();
	let mut camera_transform = q.q1_mut().single_mut().unwrap();

	camera_transform.look_at(player_transform.translation, Vec3::new(0.,1.,0.));
} */