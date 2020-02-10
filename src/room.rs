// Room.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use crate::*;

use controller::Controller;


#[derive(gd::NativeClass)]
#[inherit(gd::Node2D)]
#[register_with(Self::register_properties)]
pub struct Room {
	enemy_count: u8,
	enemies_healed: u8
}


#[methods]
impl Room {
	fn _init(_owner: gd::Node2D) -> Room {
		Room {
			enemy_count: 0,
			enemies_healed: 0
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
	pub unsafe fn _ready(&mut self, mut owner: Node2D) {
		for enemy in owner.get_tree().unwrap().get_current_scene().unwrap().get_children().iter() {
			if enemy.try_to_object::<Node>().unwrap().is_in_group("Enemy".into()) {
				self.enemy_count += 1;
			}
		}

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
		self.enemies_healed += 1;
		if self.enemies_healed >= self.enemy_count {
			get_singleton!(owner, Node, Controller).into_script().map_mut(|contr| { contr.add_room_cleared(owner.get_filename()); }).unwrap();
			owner.emit_signal("room_is_cleared".into(), &[]);
		}
	}
}
