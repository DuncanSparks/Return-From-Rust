// GroundAttack.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use player::Player;

use crate::*;


#[derive(gd::NativeClass)]
#[inherit(gd::Area2D)]
#[register_with(Self::register_properties)]
pub struct GroundAttack {
	in_area: bool
}


#[methods]
impl GroundAttack {
	fn _init(_owner: gd::Area2D) -> GroundAttack {
		GroundAttack {
			in_area: false
		}
	}

	fn register_properties(_builder: &gd::init::ClassBuilder<Self>) {
	}

	#[export]
	pub unsafe fn _on_GroundAttack_body_entered(&mut self, _owner: Area2D, body: Node) {
		if body.is_in_group("Player".into()) {
			self.in_area = true;
		}
	}

	#[export]
	pub unsafe fn _on_GroundAttack_body_exited(&mut self, _owner: Area2D, body: Node) {
		if body.is_in_group("Player".into()) {
			self.in_area = false;
		}
	}

	#[export]
	pub unsafe fn _on_AnimationPlayer_animation_finished(&mut self, mut owner: Area2D, _anim_name: GodotString) {
		owner.queue_free();
	}

	#[export]
	pub unsafe fn attack(&self, owner: Area2D) {
		get_node!(owner, AudioStreamPlayer, "SoundFire").unwrap().play(0.0);
		if self.in_area {
			let player_ref = get_singleton!(owner, KinematicBody2D, Player);
			player_ref.map_mut(|player, owner| { player.damage(owner, 2); }).unwrap();
		}
	}
}
