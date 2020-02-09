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
	pub unsafe fn _process(&mut self, mut owner: gd::RigidBody2D, _delta: f64) {
		let y = owner.get_position().y as i64;
		owner.set_z_index(y);

		//if owner.get_position().y < -10. || owner.get_position
	}

	#[export]
	pub unsafe fn _on_AreaPickUp_body_entered(&mut self, mut owner: gd::RigidBody2D, body: gd::Node) {
		if body.is_in_group("Player".into()) && self.can_pick_up {
			get_node!(owner, AudioStreamPlayer, "SoundPickUp").unwrap().play(0.0);

			let player_ref = get_instance_ref!(Player, body, KinematicBody2D).into_script();
			player_ref.map_mut(|player| {
				player.set_bullet_available(true);
			}).unwrap();
			godot_print!("PICKED UP");

			get_node!(owner, Sprite, "Sprite").unwrap().hide();
			get_node!(owner, Particles2D, "PartsIdle").unwrap().set_emitting(false);
			get_node!(owner, Particles2D, "PartsPickUp").unwrap().set_emitting(true);
			get_node!(owner, Timer, "TimerDestroy").unwrap().start(0.0);
			self.can_pick_up = false;
			self.can_hit = false;
			owner.emit_signal("picked_up".into(), &[]);
		}
	}

	#[export]
	pub unsafe fn _on_TimerPickUp_timeout(&mut self, _owner: gd::RigidBody2D) {
		self.can_pick_up = true;
	}

	#[export]
	pub unsafe fn _on_PlayerBullet_body_entered(&mut self, _owner: gd::RigidBody2D, body: gd::Node) {
		//if self.can_hit && body.is_in_group("Enemy".into()) && !get_instance_ref!(Enemy, 
	}

	#[export]
	pub unsafe fn _on_TimerStopHitting_timeout(&mut self, _owner: gd::RigidBody2D) {
		self.can_hit = false;
	}
}
