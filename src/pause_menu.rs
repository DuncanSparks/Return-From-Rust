// PauseMenu.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use crate::*;

use controller::Controller;
use player::Player;


#[derive(gd::NativeClass)]
#[inherit(gd::Node2D)]
#[register_with(Self::register_properties)]
pub struct PauseMenu {
	quitting: bool
}

#[methods]
impl PauseMenu {
	fn _init(_owner: gd::Node2D) -> PauseMenu {
		PauseMenu {
			quitting: false
		}
	}

	fn register_properties(_builder: &gd::init::ClassBuilder<Self>) {
	}
	
	#[export]
	pub unsafe fn _process(&mut self, mut owner: Node2D, _delta: f64) {
		let inp = Input::godot_singleton();
		if inp.is_action_just_pressed("sys_back".into()) {
			owner.get_tree().unwrap().set_pause(false);
			owner.queue_free();
		}
	}

	#[export]
	pub unsafe fn _on_ButtonResume_pressed(&mut self, mut owner: Node2D) {
		get_node!(owner, AudioStreamPlayer, "SoundClick").unwrap().play(0.0);
		get_singleton!(owner, KinematicBody2D, Player).map_mut(|player, owner| { player.unpause(owner); }).unwrap();
		owner.get_tree().unwrap().set_pause(false);
		owner.queue_free();
	}

	#[export]
	pub unsafe fn _on_ButtonToTitle_pressed(&mut self, owner: Node2D) {
		self.set_buttons_active(owner, false);
		get_node!(owner, AudioStreamPlayer, "SoundClick").unwrap().play(0.0);
		get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fadeout".into(), -1.0, 1.0, false);
	}

	#[export]
	pub unsafe fn _on_ButtonExit_pressed(&mut self, owner: Node2D) {
		self.set_buttons_active(owner, false);
		self.quitting = true;
		get_node!(owner, AudioStreamPlayer, "SoundClick").unwrap().play(0.0);
		get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fadeout".into(), -1.0, 1.0, false);
	}

	#[export]
	pub unsafe fn _on_AnimationPlayer_animation_finished(&mut self, mut owner: Node2D, anim_name: GodotString) {
		if anim_name == "Fadeout".into() {
			if self.quitting {
				owner.get_tree().unwrap().quit();
			}
			else {
				get_singleton!(owner, Node, Controller).into_script().map_mut(|contr| { contr.set_in_game(false); }).unwrap();

				let player_ref = get_singleton!(owner, KinematicBody2D, Player).into_script();
				let mut player_ref_2 = owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap();
				player_ref_2.hide();
				player_ref.map_mut(|player| { player.set_lock_movement(true); }).unwrap();
				get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.show_ui(owner, false); }).unwrap();
				get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.stop_music(owner); }).unwrap();

				owner.get_tree().unwrap().change_scene("res://Scenes/Title.tscn".into()).unwrap();
				owner.get_tree().unwrap().set_pause(false);
				owner.queue_free();
			}
		}
	}
	
	unsafe fn set_buttons_active(&self, owner: Node2D, active: bool) {
		get_node!(owner, Button, "CanvasLayer/NinePatchRect/ButtonResume").unwrap().set_disabled(!active);
		get_node!(owner, Button, "CanvasLayer/NinePatchRect/ButtonToTitle").unwrap().set_disabled(!active);
		get_node!(owner, Button, "CanvasLayer/NinePatchRect/ButtonExit").unwrap().set_disabled(!active);
	}
}
