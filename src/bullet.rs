// Bullet.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};
use gd::user_data::*;

use player::Player;

use crate::*;


#[derive(gd::NativeClass)]
#[inherit(gd::RigidBody2D)]
#[register_with(Self::register_properties)]
pub struct Bullet {

}


#[methods]
impl Bullet {
	const SPEED: f32 = 250.0;

	fn _init(_owner: gd::RigidBody2D) -> Bullet {
		Bullet {

		}
	}

	fn register_properties(_builder: &gd::init::ClassBuilder<Self>) {
	}

	#[export]
	pub unsafe fn _ready(&self, mut owner: RigidBody2D) {
		let ang = Angle::radians(owner.get_global_rotation() as f32);
		let ang_deg = ang.positive().to_degrees() - 90.0;
		owner.set_linear_velocity(Vector2::new(Bullet::SPEED, 0.0).rotated(-Angle::radians(ang_deg.to_radians())));
		let spr_ang = get_node!(owner, Sprite, "Sprite").unwrap().get_global_rotation() as f64;
		get_node!(owner, Sprite, "Sprite").unwrap().set_global_rotation(-spr_ang - 270.0f64.to_radians() as f64);
	}

	#[export]
	pub unsafe fn _process(&mut self, owner: RigidBody2D, _delta: f64) {
		if self.offscreen(owner) {
			self.delete(owner);
		}
	}

	#[export]
	pub unsafe fn collide(&mut self, owner: RigidBody2D, body: Node) {
		if body.is_in_group("Player".into()) {
			let player_ref = get_instance_ref!(Player, body, KinematicBody2D);
			let result = player_ref.into_script().map_mut(|player| { !player.is_in_iframes() }).unwrap();
			if result {
				let player_ref2 = get_instance_ref!(Player, body, KinematicBody2D);
				player_ref2.map_mut(|player, owner| { player.damage(owner, 1); }).unwrap();
			}
		}

		if !body.is_in_group("Bullet".into()) {
			self.delete(owner);
		}
	}

	// =====================================================================

	unsafe fn offscreen(&self, owner: RigidBody2D) -> bool {
		owner.get_position().x < -64.0 || owner.get_position().x > 384.0 || owner.get_position().y < -64.0 || owner.get_position().y > 224.0
	}

	unsafe fn delete(&self, mut owner: RigidBody2D) {
		owner.queue_free();
	}
}
