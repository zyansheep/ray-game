use bevy::prelude::*;

pub struct Player {
	pub speed: f32,
}
pub struct PlayerName(String);
#[derive(Bundle)]
pub struct PlayerBundle {
	name: PlayerName,
	_p: Player,
	#[bundle]
	model: PbrBundle,
}
impl Player {
	pub fn bundle(name: &str, model: PbrBundle) -> PlayerBundle {
		PlayerBundle {
			name: PlayerName(name.to_owned()),
			_p: Player { speed: 0.1 },
			model,
		}
	}
}
