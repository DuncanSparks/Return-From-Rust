// Fountain.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use player::Player;
use controller::Controller;

use crate::*;


#[derive(gd::NativeClass)]
#[inherit(gd::StaticBody2D)]
#[register_with(Self::register_properties)]
pub struct Fountain {
	purified: bool
}


#[methods]
impl Fountain {
	fn _init(_owner: gd::StaticBody2D) -> Fountain {
		Fountain {
			purified: false
		}
	}

	fn register_properties(_builder: &gd::init::ClassBuilder<Self>) {
	}

	#[export]
	pub unsafe fn _ready(&mut self, mut owner: StaticBody2D) {
		let y = owner.get_position().y as i64;
		owner.set_z_index(y);

		let id: GodotString = format!("{}{}{}", owner.get_tree().unwrap().get_current_scene().unwrap().get_filename().to_string(), "--", owner.get_path().to_string()).into();
		if get_singleton!(owner, Node, Controller).into_script().map(|contr| { contr.fountain_is_purified(id.new_ref()) }).unwrap() {
			self.purify(owner, true);
		}
	}

	pub unsafe fn purify(&mut self, mut owner: StaticBody2D, room_start: bool) {
		get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Purified".into(), -1.0, 1.0, false);
		if !room_start {
			get_node!(owner, AudioStreamPlayer, "SoundPurify").unwrap().play(0.0);
			get_node!(owner, Particles2D, "PartsPurify").unwrap().set_emitting(true);
			
			let player_ref = get_singleton!(owner, KinematicBody2D, Player).into_script();
			player_ref.map_mut(|player| { player.heal(5); }).unwrap();
		}

		get_node!(owner, Particles2D, "PartsFountain").unwrap().set_emitting(true);
		self.purified = true;

		let id: GodotString = format!("{}{}{}", owner.get_tree().unwrap().get_current_scene().unwrap().get_filename().to_string(), "--", owner.get_path().to_string()).into();
		let contr_ref = get_singleton!(owner, Node, Controller).into_script();
		contr_ref.map_mut(|contr| { contr.add_fountain_purified_info(id.new_ref()); }).unwrap();
	}

	pub unsafe fn is_purified(&self) -> bool {
		self.purified
	}
}
