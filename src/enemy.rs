// Enemy.rs

use gdnative as gd;
use gd::init::property;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};
use gd::user_data::*;

use crate::*;

use player::Direction;
use controller::Controller;

#[derive(gd::NativeClass)]
#[inherit(gd::KinematicBody2D)]
#[user_data(gd::user_data::LocalCellData<Enemy>)]
#[register_with(Self::register_properties)]
pub struct Enemy {
	speed: f32,
	velocity: Vector2,

	face: Direction,
	walking: bool,

	health: u16,
	healed: bool,
	in_area: bool,

	follow: bool,
	shoot: bool,
	fast_fire: bool,
	navigator: NodePath,

	ground_attack: bool,

	disappear: bool,

	nav_path: Vector2Array,

	healed_text: GodotString,

	bullet_ref: Option<PackedScene>,
	ground_attack_ref: Option<PackedScene>,
	parts_healed: Option<PackedScene>,

	timer: Option<Timer>,
	spr: Option<AnimatedSprite>,
	text: Option<RichTextLabel>,
	timer_shoot: Option<Timer>,
	timer_ground_attack: Option<Timer>,
	nav_node: Option<Navigation2D>
}


#[methods]
impl Enemy {
	fn _init(_owner: gd::KinematicBody2D) -> Enemy {
		Enemy {
			speed: 25.0,
			velocity: Vector2::zero(),

			face: Direction::Down,
			walking: false,

			health: 2,
			healed: false,
			in_area: false,

			follow: true,
			shoot: true,
			fast_fire: false,
			navigator: NodePath::new(&GodotString::new()),

			ground_attack: false,

			disappear: false,

			nav_path: Vector2Array::new(),

			healed_text: GodotString::new(),

			bullet_ref: None,
			ground_attack_ref: None,
			parts_healed: None,

			timer: None,
			spr: None,
			text: None,
			timer_shoot: None,
			timer_ground_attack: None,
			nav_node: None
		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_signal(init::Signal {
			name: "healed",
			args: &[]
		});

		builder.add_property::<f32>("speed")
		.with_default(25.0)
		.done();

		builder.add_property::<u16>("health")
		.with_default(2)
		.done();

		builder.add_property::<bool>("follow")
		.with_default(true)
		.done();

		builder.add_property::<bool>("shoot")
		.with_default(true)
		.done();

		builder.add_property::<bool>("fast_fire")
		.with_default(false)
		.done();

		builder.add_property::<bool>("ground_attack")
		.with_default(false)
		.done();

		builder.add_property::<GodotString>("healed_text")
		.with_default(GodotString::new())
		.with_hint(property::StringHint::Multiline)
		.done();

		builder.add_property::<NodePath>("navigator")
		.with_default(NodePath::new(&GodotString::new()))
		.done();
	}

	#[export]
	pub unsafe fn _ready(&mut self, mut owner: gd::KinematicBody2D) {
		self.timer = get_node!(owner, Timer, "Timer");
		self.spr = get_node!(owner, AnimatedSprite, "Sprite");
		self.text = get_node!(owner, RichTextLabel, "Text");
		self.timer_shoot = get_node!(owner, Timer, "TimerShoot");
		self.timer_ground_attack = get_node!(owner, Timer, "TimerGroundAttack");
		
		if owner.get_tree().unwrap().get_current_scene().unwrap().get_filename() == "res://Scenes/Castle/Castle_Boss.tscn".into() {
			self.disappear = true;
		}

		self.nav_node = get_node!(owner, Navigation2D, self.navigator.new_ref());

		self.timer.unwrap().set_wait_time(rand_range!(owner, 2.0, 4.0));
		self.set_text(owner, self.healed_text.new_ref());

		if self.shoot {
			self.timer_shoot.unwrap().set_wait_time(if self.fast_fire { rand_range!(owner, 0.8, 1.5) } else { rand_range!(owner, 1.5, 3.0) });
			self.timer_shoot.unwrap().start(0.0);
		}
		else if self.ground_attack {
			self.timer_ground_attack.unwrap().set_wait_time(rand_range!(owner, 2.0, 4.0));
			self.timer_ground_attack.unwrap().start(0.0);
		}

		let id: GodotString = format!("{}{}{}", owner.get_tree().unwrap().get_current_scene().unwrap().get_filename().to_string(), "--", owner.get_path().to_string()).into();
		if get_singleton!(owner, Node, Controller).into_script().map(|contr| { contr.enemy_is_healed(id.new_ref()) }).unwrap() {
			self.heal(owner, true);
			let result = get_singleton!(owner, Node, Controller).into_script().map(|contr| { contr.get_healed_enemy_position(id.new_ref()) }).unwrap();
			owner.set_position(result);
		}

		if self.follow {
			self.nav_path = self.nav_node.unwrap().get_simple_path(owner.get_global_position(), owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap().get_global_position(), false);
			get_node!(owner, Timer, "TimerNav").unwrap().start(0.0);
		}

		if self.ground_attack {
			let mat = self.spr.unwrap().get_material().unwrap().duplicate(false).unwrap().cast::<Material>();
			self.spr.unwrap().set_material(mat);

			self.spr.unwrap().get_material().unwrap().cast::<ShaderMaterial>().unwrap().set_shader_param("shift_amount".into(), 0.888.to_variant());
		}
	}

	#[export]
	pub unsafe fn _process(&mut self, mut owner: gd::KinematicBody2D, delta: f64) {
		let y = owner.get_position().y as i64;
		owner.set_z_index(y);

		if self.in_area {
			let c = self.text.unwrap().get_visible_characters();
			self.text.unwrap().set_visible_characters(c + 1);
		}

		if self.follow && !self.healed {
			self.move_along_path(self.speed * delta as f32, owner);
			self.walking = self.velocity != Vector2::zero();
		}

		if !self.healed {
			self.direction_management();
			self.sprite_management();
		}
		else {
			self.velocity = Vector2::zero();
			self.spr.unwrap().play("healed".into(), false);
		}

	}

	#[export]
	pub unsafe fn _physics_process(&mut self, mut owner: gd::KinematicBody2D, _delta: f64) {
		move_and_slide_default!(owner, self.velocity * self.speed);
	}

	#[export]
	pub unsafe fn _exit_tree(&self, _owner: KinematicBody2D) {
		deallocate!(self.bullet_ref);
		deallocate!(self.ground_attack_ref);
		deallocate!(self.parts_healed);
	}

	//#[export]
	pub unsafe fn hit(&mut self, owner: KinematicBody2D) {
		get_node!(owner, AudioStreamPlayer, "SoundHit").unwrap().play(0.0);

		let parts = self.parts_healed.as_ref().unwrap().instance(0);
		let mut parts_ref = parts.unwrap().cast::<Particles2D>().unwrap();
		parts_ref.set_position(owner.get_position());
		owner.get_tree().unwrap().get_current_scene().unwrap().add_child(parts, false);
		parts_ref.set_emitting(true);

		self.health -= 1;
		if self.health <= 0 {
			self.heal(owner, false);
		}
	}

	pub unsafe fn heal(&mut self, mut owner: KinematicBody2D, room_start: bool) {
		get_node!(owner, Particles2D, "PartsDust").unwrap().set_emitting(false);
		self.healed = true;
		owner.set_collision_mask_bit(4, false);
		get_node!(owner, CollisionShape2D, "CollisionShape2D").unwrap().call_deferred("set_disabled".into(), &[false.to_variant()]);

		if !room_start {
			get_node!(owner, AudioStreamPlayer, "SoundHeal").unwrap().play(0.0);
			if !self.disappear {
				let con = get_singleton!(owner, Node, Controller).into_script();
				con.map_mut(|contr| { contr.add_enemy_healed(); }).unwrap();
				con.map_mut(|contr| { contr.add_enemy_healed_info(format!("{}{}{}", owner.get_tree().unwrap().get_current_scene().unwrap().get_filename().to_string(), "--", owner.get_path().to_string()).into(), owner.get_position())}).unwrap();
			}
			else {
				get_node!(owner, AnimationPlayer, "AnimationPlayer").unwrap().play("Fade".into(), -1.0, 1.0, false);
				get_node!(owner, Timer, "TimerDisappear").unwrap().start(0.0);
			}

			owner.emit_signal("healed".into(), &[]);
		}
	}

	pub unsafe fn set_text(&self, owner: KinematicBody2D, value: GodotString) {
		get_node!(owner, RichTextLabel, "Text").unwrap().set_bbcode(format!("{}{}", "[wave amp=30 freq=4][center]", value.to_string()).into());
	}

	#[export]
	pub unsafe fn _on_AreaText_body_entered(&mut self, _owner: KinematicBody2D, body: Node) {
		if body.is_in_group("Player".into()) && self.healed {
			self.in_area = true;
			self.text.unwrap().set_visible_characters(0);
			self.text.unwrap().show();
		}
	}

	#[export]
	pub unsafe fn _on_AreaText_body_exited(&mut self, _owner: KinematicBody2D, body: Node) {
		if body.is_in_group("Player".into()) {
			self.in_area = false;
			self.text.unwrap().hide();
		}
	}

	#[export]
	pub unsafe fn _on_TimerShoot_timeout(&self, owner: KinematicBody2D) {
		if !self.healed {
			get_node!(owner, AudioStreamPlayer, "SoundShoot").unwrap().play(0.0);
			let bullet = self.bullet_ref.as_ref().unwrap().instance(0);
			let mut bullet_ref = bullet.unwrap().cast::<RigidBody2D>().unwrap();
			bullet_ref.set_position(owner.get_position());
			bullet_ref.set_global_rotation(owner.get_position().angle_to(owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap().get_position()).get() as f64);
			owner.get_tree().unwrap().get_current_scene().unwrap().add_child(bullet, false);
			self.timer_shoot.unwrap().set_wait_time(if self.fast_fire { rand_range!(owner, 0.8, 1.5) } else { rand_range!(owner, 1.5, 3.0) });
		}
	}

	#[export]
	pub unsafe fn _on_TimerNav_timeout(&mut self, owner: KinematicBody2D) {
		self.nav_path = self.nav_node.unwrap().get_simple_path(owner.get_global_position(), owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap().get_global_position(), false);
	}

	#[export]
	pub unsafe fn _on_TimerGroundAttack_timeout(&mut self, owner: KinematicBody2D) {
		if !self.healed {
			get_node!(owner, Particles2D, "PartsGroundAttack").unwrap().set_emitting(true);
			let inst = self.ground_attack_ref.as_ref().unwrap().instance(0);
			let mut inst_ref = inst.unwrap().cast::<Node2D>().unwrap();
			inst_ref.set_position(owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap().get_global_position() + Vector2::new(0.0, 6.0));
			owner.get_tree().unwrap().get_current_scene().unwrap().add_child(inst, false);
			self.timer_ground_attack.unwrap().set_wait_time(rand_range!(owner, 2.0, 4.0));
		}
		else {
			self.timer_ground_attack.unwrap().stop();
		}
	}
}

impl Enemy {

	// =====================================================================

	pub fn is_healed(&self) -> bool {
		self.healed
	}

	// =====================================================================

	fn direction_management(&mut self) {
		//let prev_face = self.face.clone();

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
		let mut anim = GodotString::new();
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

	unsafe fn move_along_path(&mut self, mut distance: f32, owner: KinematicBody2D) {
		let mut start_point = owner.get_global_position();
		for _ in 0..self.nav_path.len() {
			let target = self.nav_path.get(0);
			let dist = ((target.x - start_point.x).powi(2) + (target.y - start_point.y).powi(2)).sqrt();
			if distance <= dist && dist >= 0.0 {
				let angle = owner.get_position().angle_to(target).get();
				self.velocity = Vector2::new(angle.cos(), angle.sin());
				break;
			}

			distance -= dist;
			start_point = self.nav_path.get(0);
			self.nav_path.remove(0);
		}
	}
}
