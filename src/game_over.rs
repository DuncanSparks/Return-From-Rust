// GameOver.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use crate::*;

use controller::Controller;


#[derive(gd::NativeClass)]
#[inherit(gd::Node2D)]
#[register_with(Self::register_properties)]
pub struct GameOver {

}


#[methods]
impl GameOver {
	fn _init(_owner: gd::Node2D) -> GameOver {
		GameOver {

		}
	}

	fn register_properties(_builder: &gd::init::ClassBuilder<Self>) {
	}

	#[export]
	pub unsafe fn _on_Timer_timeout(&self, owner: Node2D) {
		get_singleton!(owner, Node, Controller).into_script().map_mut(|contr| { contr.set_in_game(false); }).unwrap();
		owner.get_tree().unwrap().change_scene("res://Scenes/Title.tscn".into()).unwrap();
	}

	#[export]
	pub unsafe fn _on_Timer2_timeout(&self, owner: Node2D) {
		get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fadein".into(), -1.0, 1.0, false);
		get_node!(owner, AudioStreamPlayer, "MusicGameOver").unwrap().play(0.0);
	}
}
