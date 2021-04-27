
use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*, render::camera::PerspectiveProjection};

#[derive(Clone)]
pub struct OrbitCameraInitialState {
	pub initial_focus: Vec3,
	pub initial_position: Vec3,
}
pub struct OrbitCameraPlugin { initial_state: OrbitCameraInitialState }
impl Plugin for OrbitCameraPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
		.insert_resource(self.initial_state.clone()) // Initial state of camera
		.add_event::<CameraFocusEvent>() // Focus change event listener
    	.add_startup_system(spawn_camera.system())
		.add_system_to_stage(CoreStage::PostUpdate, orbit_camera.system()); // Make sure camera is updated after main logic is run (i.e. Focus Event)
	}
}
impl OrbitCameraPlugin {
	pub fn new(initial_focus: Vec3, initial_position: Vec3) -> Self {
		Self {
			initial_state: OrbitCameraInitialState {
				initial_focus,
				initial_position,
			}
		}
	}
}

#[derive(Clone)]
pub enum CameraInterpolation {
	None,
	Linear,
	Ease,
}
/// Component that can be put on an object and will be focused on by the camera
#[derive(Clone)]
pub struct CameraFocusEvent {
	pub target: Vec3,
	pub interpolation: CameraInterpolation,
}
impl CameraFocusEvent {
	pub fn new(target: Vec3, interpolation: CameraInterpolation) -> Self { Self { target, interpolation } }
}

/// Tags an entity as capable of panning and orbiting.
pub struct OrbitCamera {
	/// The "focus point" to orbit around. It is automatically updated when panning the camera
	pub focus: Vec3,
	/// Distance from focus point
	pub distance: f32,
	/// What button to use for orbiting
	pub orbit_button: MouseButton,
	/// Optional destination focus point for interpolating smoothly to new camera focus
	pub destination_focus: Option<CameraFocusEvent>,
}

impl Default for OrbitCamera {
	fn default() -> Self {
		Self {
			focus: Vec3::ZERO,
			distance: 5.0,
			orbit_button: MouseButton::Left,
			destination_focus: None,
		}
	}
}
/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn orbit_camera(
	windows: Res<Windows>,
	mut ev_motion: EventReader<MouseMotion>,
	mut ev_scroll: EventReader<MouseWheel>,
	mut ev_focus: EventReader<CameraFocusEvent>,
	input_mouse: Res<Input<MouseButton>>,
	mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {

	let (mut orbit_camera, mut camera_transform) = query.single_mut().unwrap();
	orbit_camera.destination_focus = ev_focus.iter().last().cloned();

	let mut rotation_move = Vec2::ZERO;
	let mut scroll = 0.0;

	let mut update_camera = false;

	if input_mouse.pressed(MouseButton::Left) {
		for ev in ev_motion.iter() {
			rotation_move += ev.delta;
		}
	}
	if let Some(focus_event) = &orbit_camera.destination_focus {
		update_camera = true;
		orbit_camera.focus = match focus_event.interpolation {
			CameraInterpolation::None => focus_event.target,
			_ => { unimplemented!() }
		}
	}
	for ev in ev_scroll.iter() { scroll += ev.y; }

	if rotation_move.length_squared() > 0.0 {
		update_camera = true;
		let window = get_primary_window_size(&windows);
		let delta_x = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
		let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
		let yaw = Quat::from_rotation_y(-delta_x);
		let pitch = Quat::from_rotation_x(-delta_y);
		camera_transform.rotation = yaw * camera_transform.rotation; // rotate around global y axis
		camera_transform.rotation = camera_transform.rotation * pitch; // rotate around local x axis
	} else if scroll.abs() > 0.0 {
		update_camera = true;
		orbit_camera.distance -= scroll * orbit_camera.distance * 0.2;
		// dont allow zoom to reach zero or you get stuck
		orbit_camera.distance = f32::max(orbit_camera.distance, 0.05);
	}

	if update_camera {
		// emulating parent/child to make the yaw/y-axis rotation behave like a turntable
		// parent = x and y rotation
		// child = z-offset
		let rot_matrix = Mat3::from_quat(camera_transform.rotation);
		camera_transform.translation = orbit_camera.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbit_camera.distance));
	}
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
	let window = windows.get_primary().unwrap();
	let window = Vec2::new(window.width() as f32, window.height() as f32);
	window
}

/// Spawn a camera like this
fn spawn_camera(mut commands: Commands, initial_state: ResMut<OrbitCameraInitialState>) {
	let focus = initial_state.initial_focus;
	let translation = initial_state.initial_position - focus;
	let distance = translation.length();
	info!("translation: {}, distance: {}", translation, distance);

	commands.spawn_bundle(PerspectiveCameraBundle {
		transform: Transform::from_translation(translation)
			.looking_at(focus, Vec3::Y),
		..Default::default()
	}).insert(OrbitCamera {
		focus,
		distance,
		..Default::default()
	});
}
