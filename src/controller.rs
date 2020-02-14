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

	in_game: bool,
	speedrun_timer: bool,
	timer: f64,

	sound_oneshot_ref: Option<PackedScene>,
	pause_menu_ref: Option<PackedScene>,

	text_healed: Option<Label>,
	healthbar: Option<TextureProgress>,
	timer_text: Option<Label>,

	ui_1: Option<Control>,
	ui_alpha_target_1: f32,
	ui_2: Option<Control>,
	ui_alpha_target_2: f32,

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

			in_game: false,
			speedrun_timer: false,
			timer: 0.0,

			sound_oneshot_ref: None,
			pause_menu_ref: None,

			text_healed: None,
			healthbar: None,
			timer_text: None,

			ui_1: None,
			ui_alpha_target_1: 1.0,
			ui_2: None,
			ui_alpha_target_2: 1.0,

			rand: RandomNumberGenerator::new()
		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_property::<PackedScene>("sound_oneshot_ref")
		.with_setter(|this: &mut Self, _owner: Node,  v| this.sound_oneshot_ref = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: Node| this.sound_oneshot_ref.as_ref().unwrap().new_ref())
		.done();

		builder.add_property::<PackedScene>("pause_menu_ref")
		.with_setter(|this: &mut Self, _owner: Node,  v| this.pause_menu_ref = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: Node| this.pause_menu_ref.as_ref().unwrap().new_ref())
		.done();
	}

	#[export]
	pub unsafe fn _ready(&mut self, owner: Node) {
		self.rand.randomize();
		OS::godot_singleton().center_window();

		Input::godot_singleton().set_mouse_mode(InputMouseMode::ModeHidden as i64);

		self.text_healed = get_node!(owner, Label, "CanvasLayer/Control/Label");
		self.timer_text = get_node!(owner, Label, "CanvasLayer2/Control/TimerText");
		self.healthbar = get_node!(owner, TextureProgress, "CanvasLayer/Control/Health");
		self.ui_1 = get_node!(owner, Control, "CanvasLayer/Control");
		self.ui_2 = get_node!(owner, Control, "CanvasLayer2/Control");
	}

	#[export]
	pub unsafe fn _process(&mut self, owner: Node, delta: f64) {
		if self.speedrun_timer {
			self.timer += delta;
		}

		self.timer_text.unwrap().set_text(GodotString::from(format!("{:02}:{:.03}", self.timer.floor() as i64 / 60, self.timer % 60.0)));

		self.text_healed.unwrap().set_text(format!("Healed: {}", self.num_enemies_healed).into());

		self.ui_1.unwrap().set_modulate(Color{r: 1.0, g: 1.0, b: 1.0, a: self.ui_alpha_target_1});
		self.ui_2.unwrap().set_modulate(Color{r: 1.0, g: 1.0, b: 1.0, a: self.ui_alpha_target_2});

		let player_hp = get_singleton!(owner, KinematicBody2D, Player).into_script().map(|pl| { pl.get_health() }).unwrap();
		self.healthbar.unwrap().set_value(player_hp as f64);

		let inp = Input::godot_singleton();
		if inp.is_action_just_pressed("sys_fullscreen".into()) {
			OS::godot_singleton().set_window_fullscreen(!OS::godot_singleton().is_window_fullscreen());
		}

		if inp.is_action_just_pressed("sys_back".into()) && self.in_game && !owner.get_tree().unwrap().is_paused() {
				let pm = self.pause_menu_ref.as_ref().unwrap().instance(0);
				owner.get_tree().unwrap().get_root().unwrap().add_child(pm, false);
				owner.get_tree().unwrap().set_pause(true);
		}

		if inp.is_action_just_pressed("debug".into()) {
			for i in 0..5 {
				self.add_fountain_purified_info(format!("{}", i).into());
			}
		}
	}

	// =====================================================================

	pub unsafe fn show_ui(&mut self, owner: Node, show: bool) {
		for child in get_node!(owner, Control, "CanvasLayer/Control").unwrap().get_children().iter_mut() {
			child.try_to_object::<CanvasItem>().unwrap().set_visible(show);
		}

		if self.speedrun_timer {
			for child in get_node!(owner, Control, "CanvasLayer2/Control").unwrap().get_children().iter_mut() {
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

	pub unsafe fn after_load(&mut self, owner: Node) {
		get_node!(owner, Timer, "TimerAfterLoad").unwrap().start(0.0);
	}

	#[export]
	pub unsafe fn after_load_2(&mut self, owner: Node) {
		let player_ref = get_singleton!(owner, KinematicBody2D, Player).into_script();
		player_ref.map_mut(|player| {
			player.set_loading(false);
		}).unwrap();
	}

	#[export]
	pub unsafe fn _on_AreaUI_body_entered(&mut self, _owner: Node, body: Node) {
		if body.is_in_group("Player".into()) || body.is_in_group("Enemy".into()) || body.is_in_group("EnemyBoss".into()) || body.is_in_group("PlayerBullet".into()) {
			self.ui_alpha_target_1 = 0.2;
		}
	}

	#[export]
	pub unsafe fn _on_AreaUI_body_exited(&mut self, _owner: Node, body: Node) {
		if body.is_in_group("Player".into()) || body.is_in_group("Enemy".into()) || body.is_in_group("EnemyBoss".into()) || body.is_in_group("PlayerBullet".into()) {
			self.ui_alpha_target_1 = 1.0;
		}
	}

	#[export]
	pub unsafe fn _on_AreaUI2_body_entered(&mut self, _owner: Node, body: Node) {
		if body.is_in_group("Player".into()) || body.is_in_group("Enemy".into()) || body.is_in_group("EnemyBoss".into()) || body.is_in_group("PlayerBullet".into()) {
			self.ui_alpha_target_2 = 0.2;
		}
	}

	#[export]
	pub unsafe fn _on_AreaUI2_body_exited(&mut self, _owner: Node, body: Node) {
		if body.is_in_group("Player".into()) || body.is_in_group("Enemy".into()) || body.is_in_group("EnemyBoss".into()) || body.is_in_group("PlayerBullet".into()) {
			self.ui_alpha_target_2 = 1.0;
		}
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

	pub fn set_in_game(&mut self, value: bool) {
		self.in_game = value;
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
