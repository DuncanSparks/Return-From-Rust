// DemonKing.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};
use gd::user_data::*;

use crate::*;

use enemy::Enemy;
use player::Player;
use controller::Controller;

#[derive(gd::NativeClass)]
#[inherit(gd::KinematicBody2D)]
#[user_data(gd::user_data::LocalCellData<DemonKing>)]
#[register_with(Self::register_properties)]
pub struct DemonKing {
	/*sound_healed: Option<AudioStream>,
	sound_teleport: Option<AudioStream>,
	sound_kick: Option<AudioStream>,
	sound_heal: Option<AudioStream>,*/

	health: u16,
	healed: bool,
	bullet_count: u8,

	navigator: NodePath,

	bullet_ref: Option<PackedScene>,
	enemy_ref: Option<PackedScene>,
	ball_ref: Option<PackedScene>,

	parts_healed: Option<PackedScene>,
	parts_teleport: Option<PackedScene>,

	timer_teleport: Option<Timer>,
	timer_teleport2: Option<Timer>,
	timer_bullet: Option<Timer>,
	timer_attack: Option<Timer>
}


#[methods]
impl DemonKing {
	fn _init(_owner: gd::KinematicBody2D) -> DemonKing {
		DemonKing {
			/*sound_healed: None,
			sound_teleport: None,
			sound_kick: None,
			sound_heal: None,*/

			health: 15,
			healed: false,
			bullet_count: 0,

			navigator: NodePath::new(&GodotString::new()),

			bullet_ref: None,
			enemy_ref: None,
			ball_ref: None,

			parts_healed: None,
			parts_teleport: None,

			timer_teleport: None,
			timer_teleport2: None,
			timer_bullet: None,
			timer_attack: None
		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_signal(init::Signal {
			name: "healed",
			args: &[]
		});

		/*builder.add_property::<AudioStream>("sound_healed")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.sound_healed = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.sound_healed.as_ref().unwrap().new_ref())
		.done();

		builder.add_property::<AudioStream>("sound_teleport")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.sound_teleport = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.sound_teleport.as_ref().unwrap().new_ref())
		.done();

		builder.add_property::<AudioStream>("sound_kick")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.sound_kick = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.sound_kick.as_ref().unwrap().new_ref())
		.done();

		builder.add_property::<AudioStream>("sound_heal")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.sound_heal = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.sound_heal.as_ref().unwrap().new_ref())
		.done();*/

		builder.add_property::<PackedScene>("bullet_ref")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.bullet_ref = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.bullet_ref.as_ref().unwrap().new_ref())
		.done();

		builder.add_property::<PackedScene>("enemy_ref")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.enemy_ref = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.enemy_ref.as_ref().unwrap().new_ref())
		.done();

		builder.add_property::<PackedScene>("ball_ref")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.ball_ref = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.ball_ref.as_ref().unwrap().new_ref())
		.done();

		builder.add_property::<PackedScene>("parts_healed")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.parts_healed = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.parts_healed.as_ref().unwrap().new_ref())
		.done();

		builder.add_property::<PackedScene>("parts_teleport")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D,  v| this.parts_teleport = if v.to_variant().is_nil() { None } else { Some(v) })
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.parts_teleport.as_ref().unwrap().new_ref())
		.done();

		builder.add_property::<NodePath>("navigator")
		.with_setter(|this: &mut Self, _owner: KinematicBody2D, v| this.navigator = v)
		.with_getter(|this: &Self, _owner: KinematicBody2D| this.navigator.new_ref())
		.done();
	}

	#[export]
	pub unsafe fn _ready(&mut self, owner: gd::KinematicBody2D) {
		self.timer_teleport = get_node!(owner, Timer, "TimerTeleport");
		self.timer_teleport2 = get_node!(owner, Timer, "TimerTeleport2");
		self.timer_attack = get_node!(owner, Timer, "TimerAttack");
		self.timer_bullet = get_node!(owner, Timer, "TimerBullet");

		self.timer_attack.unwrap().set_wait_time(rand_range!(owner, 2.5, 4.0));
		self.timer_attack.unwrap().start(0.0);
	}

	#[export]
	pub unsafe fn _process(&mut self, mut owner: gd::KinematicBody2D, _delta: f64) {
		let y = owner.get_position().y as i64;
		owner.set_z_index(y);
	}

	#[export]
	pub unsafe fn hit(&mut self, owner: KinematicBody2D) {
		let parts = self.parts_healed.as_ref().unwrap().instance(0);
		let mut parts_ref = parts.unwrap().cast::<Particles2D>().unwrap();
		parts_ref.set_position(owner.get_position());
		owner.get_tree().unwrap().get_current_scene().unwrap().add_child(parts, false);
		parts_ref.set_emitting(true);

		self.health -= 1;
		if self.health <= 0 {
			//get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.play_sound_oneshot(owner, self.sound_healed.as_ref().unwrap().new_ref(), 6.0, 1.0); }).unwrap();
			get_node!(owner, AudioStreamPlayer, "SoundHealed").unwrap().play(0.0);
			get_singleton!(owner, Node, Controller).into_script().map_mut(|contr| { contr.stop_timer(); }).unwrap();
			get_singleton!(owner, KinematicBody2D, Player).into_script().map_mut(|pl| { pl.set_invincible(true); }).unwrap();
			get_singleton!(owner, KinematicBody2D, Player).into_script().map_mut(|pl| { pl.set_lock_movement(true); }).unwrap();
			get_node!(owner, AnimatedSprite, "Sprite").unwrap().play("healed".into(), false);

			owner.get_tree().unwrap().get_current_scene().unwrap().get_node("MusicBoss".into()).unwrap().cast::<AudioStreamPlayer>().unwrap().stop();

			self.timer_attack.unwrap().stop();
			self.timer_teleport.unwrap().stop();
			self.timer_teleport2.unwrap().stop();
			self.healed = true;

			owner.get_tree().unwrap().get_current_scene().unwrap().get_node("AnimationPlayer".into()).unwrap().cast::<AnimationPlayer>().unwrap().play("Fade".into(), -1.0, 1.0, false);
			owner.get_tree().unwrap().get_current_scene().unwrap().get_node("TimerEnd1".into()).unwrap().cast::<Timer>().unwrap().start(0.0);
			owner.get_tree().unwrap().get_current_scene().unwrap().get_node("TimerEnd2".into()).unwrap().cast::<Timer>().unwrap().start(0.0);
		}
		else {
			//get_singleton!(owner, Node, Controller).map(|contr, owner| { contr.play_sound_oneshot(owner, self.sound_heal.as_ref().unwrap().new_ref(), 6.0, 1.0); }).unwrap();
			get_node!(owner, AudioStreamPlayer, "SoundHit").unwrap().play(0.0);
		}
	}

	#[export]
	pub unsafe fn attack(&mut self, owner: KinematicBody2D) {
		let num: u8 = rand_range!(owner, 0.0, 4.0) as u8;
		if num == 0 || num == 3 {
			self.bullet_count = 0;

			self.timer_bullet.unwrap().set_wait_time(0.1);
			self.timer_bullet.unwrap().start(0.0);
			return;
		}
		else if num == 1 {
			//get_singleton!(owner, Node, Controller).map(|contr, owner| { contr.play_sound_oneshot(owner, self.sound_teleport.as_ref().unwrap().new_ref(), 4.0, 1.0); }).unwrap();
			get_node!(owner, AudioStreamPlayer, "SoundTeleport2").unwrap().play(0.0);
			for _ in 0..rand_range!(owner, 1.0, 2.0).round() as u8 {
				let pos = Vector2::new(rand_range!(owner, 30.0, 290.0) as f32, rand_range!(owner, 30.0, 150.0) as f32);

				let parts = self.parts_teleport.as_ref().unwrap().instance(0);
				let mut parts_ref = parts.unwrap().cast::<Particles2D>().unwrap();
				parts_ref.set_position(pos);
				owner.get_tree().unwrap().get_current_scene().unwrap().add_child(parts, false);
				parts_ref.set_emitting(true);

				let enemy = self.enemy_ref.as_ref().unwrap().instance(0);
				let mut enemy_r = enemy.unwrap().cast::<KinematicBody2D>().unwrap();
				enemy_r.set_position(pos);
				get_instance_ref!(Enemy, enemy.unwrap(), KinematicBody2D).into_script().map_mut(|en| { en.set_speed(rand_range!(owner, 40.0, 60.0) as f32); }).unwrap();
				get_instance_ref!(Enemy, enemy.unwrap(), KinematicBody2D).into_script().map_mut(|en| { en.set_navigator(self.navigator.new_ref()); }).unwrap();

				owner.get_tree().unwrap().get_current_scene().unwrap().add_child(enemy, false);
			}
		}
		else {
			//get_singleton!(owner, Node, Controller).map(|contr, owner| { contr.play_sound_oneshot(owner, self.sound_kick.as_ref().unwrap().new_ref(), 0.0, 1.0); }).unwrap();
			get_node!(owner, AudioStreamPlayer, "SoundKick").unwrap().play(0.0);
			let inst = self.ball_ref.as_ref().unwrap().instance(0);
			let mut inst_r = inst.unwrap().cast::<RigidBody2D>().unwrap();
			inst_r.set_position(owner.get_position());
			
			let player_ref = owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap();
			let vec = (player_ref.get_global_position() - owner.get_global_position()).normalize();
			let angle = vec.x.atan2(vec.y) as f64;
			inst.unwrap().cast::<RigidBody2D>().unwrap().set_global_rotation(angle);

			owner.get_tree().unwrap().get_current_scene().unwrap().add_child(inst, false);
		}

		self.timer_teleport.unwrap().set_wait_time(rand_range!(owner, 0.5, 1.0));
		self.timer_teleport.unwrap().start(0.0);
	}

	#[export]
	pub unsafe fn tele_out(&self, owner: KinematicBody2D) {
		//get_singleton!(owner, Node, Controller).map(|contr, owner| { contr.play_sound_oneshot(owner, self.sound_teleport.as_ref().unwrap().new_ref(), 4.0, 1.0); }).unwrap();
		get_node!(owner, AudioStreamPlayer, "SoundTeleport").unwrap().play(0.0);

		let parts = self.parts_teleport.as_ref().unwrap().instance(0);
		let mut parts_ref = parts.unwrap().cast::<Particles2D>().unwrap();
		parts_ref.set_position(owner.get_position());
		owner.get_tree().unwrap().get_current_scene().unwrap().add_child(parts, false);
		parts_ref.set_emitting(true);

		get_node!(owner, AnimatedSprite, "Sprite").unwrap().hide();
		get_node!(owner, CollisionShape2D, "CollisionShape2D").unwrap().set_disabled(true);
		self.timer_teleport2.unwrap().set_wait_time(rand_range!(owner, 1.0, 2.0));
		self.timer_teleport2.unwrap().start(0.0);
	}

	#[export]
	pub unsafe fn tele_in(&self, mut owner: KinematicBody2D) {
		//get_singleton!(owner, Node, Controller).map(|contr, owner| { contr.play_sound_oneshot(owner, self.sound_teleport.as_ref().unwrap().new_ref(), 4.0, 1.0); }).unwrap();
		get_node!(owner, AudioStreamPlayer, "SoundTeleport").unwrap().play(0.0);

		let ow = owner;
		owner.set_position(Vector2::new(rand_range!(ow, 52.0, 268.0) as f32, rand_range!(ow, 52.0, 140.0) as f32));

		let parts = self.parts_teleport.as_ref().unwrap().instance(0);
		let mut parts_ref = parts.unwrap().cast::<Particles2D>().unwrap();
		parts_ref.set_position(owner.get_position());
		owner.get_tree().unwrap().get_current_scene().unwrap().add_child(parts, false);
		parts_ref.set_emitting(true);

		get_node!(owner, AnimatedSprite, "Sprite").unwrap().show();
		get_node!(owner, CollisionShape2D, "CollisionShape2D").unwrap().set_disabled(false);
		self.timer_attack.unwrap().set_wait_time(rand_range!(owner, 0.5, 1.0));
		self.timer_attack.unwrap().start(0.0);
	}

	#[export]
	pub unsafe fn fire_bullet(&mut self, owner: KinematicBody2D) {
		//get_singleton!(owner, Node, Controller).map(|contr, owner| { contr.play_sound_oneshot(owner, self.sound_bullet.as_ref().unwrap().new_ref(), 6.0, 1.0); }).unwrap();
		let bullet = self.bullet_ref.as_ref().unwrap().instance(0);
		let mut bullet_r = bullet.unwrap().cast::<RigidBody2D>().unwrap();
		bullet_r.set_position(owner.get_position());
		
		let player_ref = owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap();
		//bullet_r.set_global_rotation(owner.get_global_position().angle_to(player_ref.get_global_position()).get() as f64);
		let vec = (player_ref.get_global_position() - owner.get_global_position()).normalize();
		let angle = vec.x.atan2(vec.y) as f64;
		bullet.unwrap().cast::<RigidBody2D>().unwrap().set_global_rotation(angle);
		
		owner.get_tree().unwrap().get_current_scene().unwrap().add_child(bullet, false);

		self.bullet_count += 1;
		if self.bullet_count < 5 {
			self.timer_bullet.unwrap().set_wait_time(rand_range!(owner, 0.35, 0.55));
			self.timer_bullet.unwrap().start(0.0);
		}
		else {
			self.timer_teleport.unwrap().set_wait_time(rand_range!(owner, 0.5, 1.0));
			self.timer_teleport.unwrap().start(0.0);
		}
	}

	#[export]
	pub unsafe fn end_game(&self, owner: KinematicBody2D) {
		owner.get_tree().unwrap().quit();
	}

	pub fn is_healed(&self) -> bool {
		self.healed
	}
}
