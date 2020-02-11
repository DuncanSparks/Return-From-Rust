// PlayerBullet.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};
use gd::user_data::*;

use crate::*;

use player::Player;

use std::collections::HashMap;


#[derive(gd::NativeClass)]
#[inherit(gd::Node)]
#[user_data(gd::user_data::LocalCellData<Controller>)]
#[register_with(Self::register_properties)]
pub struct Controller {
	num_enemies_healed: u16,

	rooms_cleared: Vec<GodotString>,
	enemies_healed: HashMap<GodotString, Vector2>,
	fountains_purified: HashMap<GodotString, bool>,

	speedrun_timer: bool,
	timer: f64,

	sound_oneshot_ref: Option<PackedScene>,

	text_healed: Option<Label>,
	healthbar: Option<TextureProgress>,
	timer_text: Option<Label>,
	cursor: Option<Sprite>,

	rand: RandomNumberGenerator
}

#[methods]
impl Controller {
	fn _init(_owner: gd::Node) -> Controller {
		Controller {
			num_enemies_healed: 0,

			rooms_cleared: Vec::new(),
			enemies_healed: HashMap::new(),
			fountains_purified: HashMap::new(),

			speedrun_timer: false,
			timer: 0.0,

			sound_oneshot_ref: None,

			text_healed: None,
			healthbar: None,
			timer_text: None,
			cursor: None,

			rand: RandomNumberGenerator::new()
		}
	}

	fn register_properties(_builder: &gd::init::ClassBuilder<Self>) {
		
	}

	#[export]
	pub unsafe fn _ready(&mut self, owner: Node) {
		self.rand.randomize();
		OS::godot_singleton().center_window();

		Input::godot_singleton().set_mouse_mode(InputMouseMode::ModeHidden as i64);

		self.sound_oneshot_ref = load!("res://Prefabs/SoundOneShot.tscn");

		self.text_healed = get_node!(owner, Label, "CanvasLayer/Label");
		self.timer_text = get_node!(owner, Label, "CanvasLayer2/TimerText");
		self.healthbar = get_node!(owner, TextureProgress, "CanvasLayer/Health");
		self.cursor = get_node!(owner, Sprite, "CanvasLayer3/Cursor");
	}

	#[export]
	pub unsafe fn _process(&mut self, owner: Node, delta: f64) {
		if self.speedrun_timer {
			self.timer += delta;
		}

		self.cursor.unwrap().set_position(owner.get_viewport().unwrap().get_mouse_position());

		self.timer_text.unwrap().set_text(GodotString::from(self.timer.to_string()));

		self.text_healed.unwrap().set_text(format!("Healed: {}", self.num_enemies_healed).into());
		
		let player_hp = get_singleton!(owner, KinematicBody2D, Player).into_script().map(|pl| { pl.get_health() }).unwrap();
		self.healthbar.unwrap().set_value(player_hp as f64);

		let inp = Input::godot_singleton();
		if inp.is_action_just_pressed("sys_fullscreen".into()) {
			OS::godot_singleton().set_window_fullscreen(!OS::godot_singleton().is_window_fullscreen());
		}
	}

	#[export]
	pub unsafe fn _exit_tree(&self, _owner: Node) {
		deallocate!(self.sound_oneshot_ref);
	}

	// =====================================================================

	pub unsafe fn show_ui(&mut self, owner: Node, show: bool) {
		for child in get_node!(owner, CanvasLayer, "CanvasLayer").unwrap().get_children().iter_mut() {
			child.try_to_object::<CanvasItem>().unwrap().set_visible(show);
		}

		if self.speedrun_timer {
			for child in get_node!(owner, CanvasLayer, "CanvasLayer2").unwrap().get_children().iter_mut() {
				child.try_to_object::<CanvasItem>().unwrap().set_visible(show);
			}
		}
	}

	pub unsafe fn play_music(&self, owner: Node) {
		get_node!(owner, AudioStreamPlayer, "MusicDistorted").unwrap().play(0.0);
	}

	pub unsafe fn music_is_playing(&self, owner: Node) -> bool {
		get_node!(owner, AudioStreamPlayer, "MusicDistorted").unwrap().is_playing()
	}

	pub unsafe fn stop_music(&self, owner: Node) {
		get_node!(owner, AudioStreamPlayer, "MusicDistorted").unwrap().stop();
	}

	pub unsafe fn play_sound_oneshot(&self, owner: Node, sound: Option<AudioStream>, volume: f64, pitch: f64) {
		let sb = self.sound_oneshot_ref.as_ref().unwrap().instance(0);
		let mut sb_ref = sb.unwrap().cast::<AudioStreamPlayer>().unwrap();
		sb_ref.set_stream(sound);
		sb_ref.set_volume_db(volume);
		sb_ref.set_pitch_scale(pitch);
		owner.get_tree().unwrap().get_root().unwrap().add_child(sb, false);
		sb_ref.play(0.0);
	}

	pub unsafe fn after_load(&mut self, owner: Node) {
		godot_print!("MARK 8");
		/*let timer = get_node!(owner, Timer, "TimerAfterLoad");
		timer.unwrap().set_wait_time(0.2);
		timer.unwrap().start(0.0);*/
		get_node!(owner, Timer, "TimerAfterLoad").unwrap().start(0.0);
	}

	#[export]
	pub unsafe fn after_load_2(&mut self, owner: Node) {
		godot_print!("MARK 10");
		let player_ref = get_singleton!(owner, KinematicBody2D, Player).into_script();
		godot_print!("MARK 11");
		player_ref.map_mut(|player| {
			player.set_loading(false);
		}).unwrap();

		godot_print!("MARK 12");
	}
}

	// =====================================================================

impl Controller {
	pub fn is_speedrun_timer_on(&self) -> bool {
		self.speedrun_timer
	}

	pub fn set_speedrun_timer(&mut self, value: bool) {
		self.speedrun_timer = value;
	}

	pub fn reset(&mut self) {
		self.rooms_cleared.clear();
		self.enemies_healed.clear();
		self.fountains_purified.clear();
		self.num_enemies_healed = 0;
		self.timer = 0.0;
	}

	pub fn add_enemy_healed(&mut self) {
		self.num_enemies_healed += 1;
	}

	pub fn add_enemy_healed_info(&mut self, enemy_id: GodotString, enemy_pos: Vector2) {
		self.enemies_healed.insert(enemy_id, enemy_pos);
	}

	pub fn add_fountain_purified_info(&mut self, fountain_id: GodotString) {
		self.fountains_purified.insert(fountain_id, true);
	}

	pub fn stop_timer(&mut self) {
		self.speedrun_timer = false;
	}

	pub fn rand_range(&mut self, from: f64, to: f64) -> f64 {
		self.rand.randf_range(from, to)
	}

	pub fn enemy_is_healed(&self, enemy_id: GodotString) -> bool {
		self.enemies_healed.contains_key(&enemy_id)
	}

	pub fn get_healed_enemy_position(&self, enemy_id: GodotString) -> Vector2 {
		self.enemies_healed[&enemy_id]
	}

	pub fn fountain_is_purified(&self, fountain_id: GodotString) -> bool {
		self.fountains_purified.contains_key(&fountain_id)
	}

	pub fn all_fountains_purified(&self) -> bool {
		self.fountains_purified.len() >= 5
	}

	pub fn add_room_cleared(&mut self, room: GodotString) {
		self.rooms_cleared.push(room);
	}

	pub fn is_room_cleared(&self, room: GodotString) -> bool {
		self.rooms_cleared.contains(&room)
	}
}
