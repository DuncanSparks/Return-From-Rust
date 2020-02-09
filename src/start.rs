// Start.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};
use gd::user_data::*;

use crate::*;

use player::Player;
use controller::Controller;


#[derive(gd::NativeClass)]
#[inherit(gd::Node2D)]
#[register_with(Self::register_properties)]
pub struct Start {

}


#[methods]
impl Start {
	fn _init(_owner: gd::Node2D) -> Start {
		Start {

		}
	}

	fn register_properties(_builder: &gd::init::ClassBuilder<Self>) {
	}

	#[export]
	pub unsafe fn _ready(&mut self, owner: Node2D) {
		owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap().hide();
		get_singleton!(owner, KinematicBody2D, Player).into_script().map_mut(|player| { player.set_lock_movement(true); }).unwrap();
		get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.show_ui(owner, false); }).unwrap();
	}

	#[export]
	pub unsafe fn start(&self, owner: Node2D) {
		owner.get_tree().unwrap().change_scene("res://Scenes/Title.tscn".into()).unwrap();
	}
}
