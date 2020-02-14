// Player.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use crate::*;

use std::cmp;

use controller::Controller;


#[derive(Clone)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right
}

impl Direction {
	pub fn from_u8(value: u8) -> Direction {
		match value {
			0 => Direction::Up,
			1 => Direction::Down,
			2 => Direction::Left,
			3 => Direction::Right,
			_ => Direction::Up
		}
	}
}


#[derive(gd::NativeClass)]
#[inherit(gd::KinematicBody2D)]
#[user_data(gd::user_data::LocalCellData<Player>)]
#[register_with(Self::register_properties)]
pub struct Player {
	velocity: Vector2,
	face: Direction,
	walking: bool,

	health: u16,
	iframes: bool,
	invincible: bool,

	loading: bool,
	lock_movement: bool,

	bullet_available: bool,
	click_lock: bool,

	bullet_ref: Option<PackedScene>,

	spr: Option<AnimatedSprite>,
	sound_kick: Option<AudioStreamPlayer>
}


#[methods]
impl Player {
	const SPEED: f32 = 100.0;

	fn _init(_owner: gd::KinematicBody2D) -> Player {
		Player {
			velocity: Vector2::zero(),
			face: Direction::Down,
			walking: false,

			health: 5,
			iframes: false,
			invincible: false,

			loading: false,
			lock_movement: false,

			bullet_available: true,
			click_lock: false,

			bullet_ref: None,

			spr: None,
			sound_kick: None,
		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_property::<PackedScene>("bullet_ref")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.bullet_ref =  if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.bullet_ref.as_ref().unwrap().new_ref())
		.done();
	}

	#[export]
	pub unsafe fn _ready(&mut self, mut owner: gd::KinematicBody2D) {
		self.spr = get_node!(owner, AnimatedSprite, "Sprite");
		self.sound_kick = get_node!(owner, AudioStreamPlayer, "SoundKick");

		owner.set_position(Vector2::new(160.0, 120.0));
	}

	#[export]
	pub unsafe fn _process(&mut self, mut owner: gd::KinematicBody2D, _delta: f64) {
		let y = owner.get_position().y as i64 + 8;
		owner.set_z_index(y);

		let inp = Input::godot_singleton();

		if !self.lock_movement {
			let x1: i8 = if inp.is_action_pressed("move_right".into()) { 1 } else { 0 };
			let x2: i8 = if inp.is_action_pressed("move_left".into()) { 1 } else { 0 };
			let y1: i8 = if inp.is_action_pressed("move_down".into()) { 1 } else { 0 };
			let y2: i8 = if inp.is_action_pressed("move_up".into()) { 1 } else { 0 };

			self.velocity.x = (x1 - x2) as f32;
			self.velocity.y = (y1 - y2) as f32;

			self.walking = self.velocity != Vector2::zero();

			self.direction_management();
			self.sprite_management();

			if inp.is_action_just_pressed("attack".into()) && self.bullet_available && !self.click_lock {
				self.throw_bullet(owner);
				self.bullet_available = false;
			}
		}
		else {
			self.velocity = Vector2::zero();
			self.walking = false;
		}
	}

	#[export]
	pub unsafe fn _physics_process(&mut self, mut owner: gd::KinematicBody2D, _delta: f64) {
		move_and_slide_default!(owner, self.velocity * Player::SPEED);
	}

	pub unsafe fn damage(&mut self, mut owner: KinematicBody2D, amount: u16) {
		if !self.invincible {
			get_node!(owner, AudioStreamPlayer, "SoundHurt").unwrap().play(0.0);
			self.health -= amount;
			self.iframes = true;
			get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("IFrames".into(), -1.0, 1.0, false);
			if self.health <= 0 {
				owner.hide();
				self.lock_movement = true;
				get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.show_ui(owner, false); }).unwrap();
				get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.stop_music(owner); }).unwrap();
				owner.get_tree().unwrap().change_scene("res://Scenes/GameOver.tscn".into()).unwrap();
			}
		}
	}

	#[export]
	pub unsafe fn _on_AnimationPlayer_animation_finished(&mut self, _owner: KinematicBody2D, anim_name: GodotString) {
		if anim_name == "IFrames".into() {
			self.iframes = false;
		}
	}

	#[export]
	pub unsafe fn unpause(&mut self, owner: KinematicBody2D) {
		self.click_lock = true;
		get_node!(owner, Timer, "TimerPause").unwrap().start(0.0);
	}

	#[export]
	pub unsafe fn after_unpause(&mut self, _owner: KinematicBody2D) {
		self.click_lock = false;
	}
}

impl Player {

	// =====================================================================

	pub fn set_face(&mut self, value: Direction) {
		self.face = value;
	}

	pub fn set_bullet_available(&mut self, value: bool) {
		self.bullet_available = value;
	}

	pub fn heal(&mut self, amount: u16) {
		self.health = cmp::min(self.health + amount, 5);
	}

	pub fn is_loading(&self) -> bool {
		self.loading
	}

	pub fn set_loading(&mut self, value: bool) {
		self.loading = value;
	}

	pub fn get_health(&self) -> u16 {
		self.health
	}

	pub fn set_lock_movement(&mut self, value: bool) {
		self.lock_movement = value;
	}

	pub fn is_in_iframes(&self) -> bool {
		self.iframes
	}

	pub fn set_invincible(&mut self, value: bool) {
		self.invincible = value;
	}

	// =====================================================================

	fn direction_management(&mut self) {
		if self.velocity.x == 0.0 {
			match self.velocity.y as i8 {
				-1 => self.face = Direction::Up,
				1 => self.face = Direction::Down,
				_ => {}
			}
		}
		else if self.velocity.y == 0.0 {
			match self.velocity.x as i8 {
				-1 => self.face = Direction::Left,
				1 => self.face = Direction::Right,
				_ => {}
			}
		}
	}

	unsafe fn sprite_management(&mut self) {
		let mut anim: GodotString;
		match self.face {
			Direction::Up => anim = "up".into(),
			Direction::Down => anim = "down".into(),
			Direction::Left => anim = "left".into(),
			Direction::Right => anim = "right".into()
		}

		if self.walking {
			anim = format!("{}{}", anim.to_string(), "_walk").into();
		}

		self.spr.unwrap().play(anim, false);
	}

	unsafe fn throw_bullet(&mut self, owner: KinematicBody2D) {
		let bullet = self.bullet_ref.as_ref().unwrap().instance(0);
		bullet.unwrap().cast::<RigidBody2D>().unwrap().set_position(owner.get_position() + Vector2::new(0.0, 5.0));

		let vec = (owner.get_global_mouse_position() - owner.get_position()).normalize();
		let angle = vec.x.atan2(vec.y) as f64;
		bullet.unwrap().cast::<RigidBody2D>().unwrap().set_global_rotation(angle);

		owner.get_tree().unwrap().get_current_scene().unwrap().add_child(bullet, false);
		self.sound_kick.unwrap().play(0.0);
	}
}
