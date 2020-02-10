// Room.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use crate::*;

use controller::Controller;
use enemy::Enemy;


#[derive(gd::NativeClass)]
#[inherit(gd::Node2D)]
#[register_with(Self::register_properties)]
pub struct Room {

}


#[methods]
impl Room {
	fn _init(_owner: gd::Node2D) -> Room {
		Room {

		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_signal(init::Signal {
			name: "room_is_cleared",
			args: &[]
		});

		builder.add_signal(init::Signal {
			name: "fountains_cleared",
			args: &[]
		});
	}

	#[export]
	pub unsafe fn _ready(&self, mut owner: Node2D) {
		let contr_ref = get_singleton!(owner, Node, Controller).into_script();
		let id = owner.get_filename();
		if contr_ref.map(|contr| { contr.is_room_cleared(id.new_ref()) }).unwrap() {
			owner.emit_signal("room_is_cleared".into(), &[]);
		}

		if contr_ref.map(|contr| { contr.all_fountains_purified() }).unwrap() {
			owner.emit_signal("fountains_cleared".into(), &[]);
		}

		if id == "res://Scenes/Castle/Castle_1.tscn".into() {
			get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.stop_music(owner); }).unwrap();
		}

		if id == "res://Scenes/Overworld/Overworld_4.tscn".into() && !get_singleton!(owner, Node, Controller).map(|contr, owner| { contr.music_is_playing(owner) }).unwrap() {
			get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.play_music(owner); }).unwrap();
		}
	}

	#[export]
	pub unsafe fn clear_room(&mut self, mut owner: Node2D) {
		let mut cleared = true;
		for enemy in owner.get_tree().unwrap().get_nodes_in_group("Enemy".into()).iter() {
			let e = Instance::<Enemy>::try_from_unsafe_base(enemy.try_to_object::<KinematicBody2D>().unwrap());
			if !e.unwrap().into_script().map(|en| { en.is_healed() }).unwrap() {
				cleared = false;
			}
		}

		if cleared {
			get_singleton!(owner, Node, Controller).into_script().map_mut(|contr| { contr.add_room_cleared(owner.get_filename()); }).unwrap();
			if owner.get_filename() != "res://Scenes/Overworld/Overworld_1.tscn".into() {
				owner.emit_signal("room_is_cleared".into(), &[]);
			}
		}
	}
}
