// PlayerBullet.rs

use gdnative as gd;
use gd::init::property;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};
use gd::user_data::*;

use player::Player;

use crate::*;


#[derive(gd::NativeClass)]
#[inherit(gd::RigidBody2D)]
#[user_data(gd::user_data::LocalCellData<PlayerBullet>)]
#[register_with(Self::register_properties)]
pub struct PlayerBullet {
	stopped: bool,

	can_pick_up: bool,
	can_hit: bool,
}


#[methods]
impl PlayerBullet {
	const SPEED: f32 = 250.0;

	fn _init(_owner: gd::RigidBody2D) -> PlayerBullet {
		PlayerBullet {
			stopped: false,
			can_pick_up: false,
			can_hit: true
		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_property::<bool>("can_pick_up")
		.with_default(false)
		.with_usage(property::Usage::DEFAULT)
		.done();

		builder.add_signal(init::Signal {
			name: "picked_up",
			args: &[]
		});

	}

	#[export]
	pub unsafe fn _ready(&self, mut owner: gd::RigidBody2D) {
		if !self.stopped {
			owner.set_linear_velocity(Vector2::new(PlayerBullet::SPEED, 0.0));
		}
		else {
			owner.set_angular_velocity(0.0);
		}
	}

	#[export]
	pub unsafe fn _process(&mut self, mut owner: gd::RigidBody2D, delta: f64) {
		let y = owner.get_position().y as i64;
		owner.set_z_index(y);

		//if owner.get_position().y < -10. || owner.get_position
	}

	#[export]
	pub unsafe fn _on_AreaPickUp_body_entered(&self, owner: gd::RigidBody2D, body: gd::Node) {
		if body.is_in_group("Player".into()) && self.can_pick_up {
			get_node!(owner, AudioStreamPlayer, "SoundPickUp").unwrap().play(0.0);
			let player_ref = get_instance_ref!(Player, body, KinematicBody2D).into_script();
			player_ref.map_mut(|player| {
				player.set_bullet_available(true);
			}).unwrap();
		}
	}
}
