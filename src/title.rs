// Title.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};
use gd::user_data::*;

use crate::*;

use player::Player;
use controller::Controller;


#[derive(gd::NativeClass)]
#[inherit(gd::Node2D)]
#[register_with(Self::register_properties)]
pub struct Title {
	credits_open: bool,
	quitting: bool
}

#[methods]
impl Title {
	fn _init(_owner: gd::Node2D) -> Title {
		Title {
			credits_open: false,
			quitting: false
		}
	}

	fn register_properties(_builder: &gd::init::ClassBuilder<Self>) {
	}

	#[export]
	pub unsafe fn _ready(&mut self, owner: Node2D) {
		let sr_timer = get_singleton!(owner, Node, Controller).into_script().map(|contr| { contr.is_speedrun_timer_on()} ).unwrap();
		get_node!(owner, Button, "ButtonSpeedrunTimer").unwrap().set_text(if sr_timer { "Speedrun Timer - On".into() } else { "Speedrun Timer - Off".into() });
	}

	#[export]
	pub unsafe fn _process(&mut self, owner: Node2D, _delta: f64) {
		let inp = Input::godot_singleton();

		if inp.is_action_just_pressed("attack".into()) && self.credits_open {
			self.credits_open = false;
			get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fadeout Credits".into(), -1.0, 1.0, false);
		}
	}

	#[export]
	pub unsafe fn _on_ButtonStart_pressed(&mut self, owner: Node2D) {
		self.set_buttons_active(owner, false);
		get_node!(owner, AudioStreamPlayer, "SoundClick").unwrap().play(0.0);
		get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fadeout".into(), -1.0, 1.0, false);
	}

	#[export]
	pub unsafe fn _on_ButtonCredits_pressed(&mut self, owner: Node2D) {
		self.set_buttons_active(owner, false);
		get_node!(owner, AudioStreamPlayer, "SoundClick").unwrap().play(0.0);
		get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fadeout Keep Music".into(), -1.0, 1.0, false);
	}

	#[export]
	pub unsafe fn _on_ButtonExit_pressed(&mut self, owner: Node2D) {
		self.set_buttons_active(owner, false);
		self.quitting = true;
		get_node!(owner, AudioStreamPlayer, "SoundClick").unwrap().play(0.0);
		get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fadeout".into(), -1.0, 1.0, false);
	}

	#[export]
	pub unsafe fn _on_ButtonSpeedrunTimer_pressed(&mut self, owner: Node2D) {
		get_node!(owner, AudioStreamPlayer, "SoundClick2").unwrap().play(0.0);
		let contr_ref = get_singleton!(owner, Node, Controller).into_script();
		let sr_value = contr_ref.map(|contr| { contr.is_speedrun_timer_on() }).unwrap();
		contr_ref.map_mut(|contr| { contr.set_speedrun_timer(!sr_value); }).unwrap();
		get_node!(owner, Button, "ButtonSpeedrunTimer").unwrap().set_text(if !sr_value { "Speedrun Timer - On".into() } else { "Speedrun Timer - Off".into() });
	}

	#[export]
	pub unsafe fn _on_AnimationPlayer_animation_finished(&mut self, owner: Node2D, anim_name: GodotString) {
		if anim_name == "Fadeout".into() {
			if self.quitting {
				owner.get_tree().unwrap().quit();
			}
			else {
				self.start_game(owner);
			}
		}
		else if anim_name == "Fadein".into() {
			self.set_buttons_active(owner, true);
		}
		else if anim_name == "Fadeout Credits".into() {
			get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fadein".into(), -1.0, 1.0, false);
		}
		else if anim_name == "Fadeout Keep Music".into() {
			get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fadein Credits".into(), -1.0, 1.0, false);
		}
		else if anim_name == "Fadein Credits".into() {
			self.credits_open = true;
		}
	}

	unsafe fn set_buttons_active(&self, owner: Node2D, active: bool) {
		get_node!(owner, Button, "ButtonStart").unwrap().set_disabled(!active);
		get_node!(owner, Button, "ButtonCredits").unwrap().set_disabled(!active);
		get_node!(owner, Button, "ButtonExit").unwrap().set_disabled(!active);
		get_node!(owner, Button, "ButtonSpeedrunTimer").unwrap().set_disabled(!active);
	}

	unsafe fn start_game(&mut self, owner: Node2D) {
		owner.get_tree().unwrap().change_scene("res://Scenes/Overworld/Overworld_1.tscn".into()).unwrap();
		let player_ref = get_singleton!(owner, KinematicBody2D, Player).into_script();
		player_ref.map_mut(|player| { player.set_lock_movement(false); }).unwrap();
		get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.show_ui(owner, true); }).unwrap();
		let mut player_ref_2 = owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap();
		player_ref_2.set_position(Vector2::new(160.0, 120.0));
		player_ref_2.show();
		player_ref.map_mut(|player| { player.heal(10); }).unwrap();

		let contr_ref = get_singleton!(owner, Node, Controller).into_script();
		get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.play_music(owner); }).unwrap();
		contr_ref.map_mut(|contr| { contr.reset(); }).unwrap();
		contr_ref.map_mut(|contr| { contr.set_in_game(true); }).unwrap();
	}
}
