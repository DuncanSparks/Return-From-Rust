// PlayerBullet.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};
use gd::user_data::*;

use player::Player;
use enemy::Enemy;
use fountain::Fountain;

use crate::*;

use demon_king::DemonKing;


#[derive(gd::NativeClass)]
#[inherit(gd::RigidBody2D)]
#[user_data(gd::user_data::LocalCellData<PlayerBullet>)]
#[register_with(Self::register_properties)]
pub struct PlayerBullet {
	stopped: bool,

	can_pick_up: bool,
	can_hit: bool,
	in_area: bool
}


#[methods]
impl PlayerBullet {
	const SPEED: f32 = 250.0;

	fn _init(_owner: gd::RigidBody2D) -> PlayerBullet {
		PlayerBullet {
			stopped: false,
			can_pick_up: false,
			can_hit: true,
			in_area: false
		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_property::<bool>("stopped")
		.with_default(false)
		.with_setter(|this: &mut Self, _owner: RigidBody2D,  v| this.stopped = v)
		.with_getter(|this: &Self, _owner: RigidBody2D| this.stopped)
		.done();

		builder.add_signal(init::Signal {
			name: "picked_up",
			args: &[]
		});
	}

	#[export]
	pub unsafe fn _ready(&mut self, mut owner: gd::RigidBody2D) {
		if !self.stopped {
			let ang = Angle::radians(owner.get_global_rotation() as f32);
			let ang_deg = ang.positive().to_degrees() - 90.0;
			owner.set_linear_velocity(Vector2::new(PlayerBullet::SPEED, 0.0).rotated(-Angle::radians(ang_deg.to_radians())));
			get_node!(owner, Timer, "TimerStopHitting").unwrap().start(0.0);
		}
		else {
			owner.set_angular_velocity(0.0);
			get_node!(owner, Timer, "TimerPickUp").unwrap().set_wait_time(0.05);
			get_node!(owner, Timer, "TimerPickUp").unwrap().start(0.0);
		}
	}

	#[export]
	pub unsafe fn _process(&mut self, mut owner: gd::RigidBody2D, _delta: f64) {
		let y = owner.get_position().y as i64;
		owner.set_z_index(y);

		if owner.get_position().x < -10.0 || owner.get_position().x > 330.0 || owner.get_position().y < -10.0 || owner.get_position().y > 190.0 {
			get_singleton!(owner, KinematicBody2D, Player).into_script().map_mut(|player| { player.set_bullet_available(true); }).unwrap();
			owner.queue_free();
		}
	}

	#[export]
	pub unsafe fn _on_AreaPickUp_body_entered(&mut self, owner: RigidBody2D, body: gd::Node) {
		if body.is_in_group("Player".into()) {
			if self.can_pick_up {
				self.pick_up(owner)
			}
			else {
				self.in_area = true;
			}
		}
	}

	#[export]
	pub unsafe fn _on_AreaPickUp_body_exited(&mut self, _owner: RigidBody2D, body: gd::Node) {
		if body.is_in_group("Player".into()) {
			self.in_area = false;
		}
	}

	unsafe fn pick_up(&mut self, mut owner: RigidBody2D) {
		get_node!(owner, AudioStreamPlayer, "SoundPickUp").unwrap().play(0.0);

		let player_ref = get_singleton!(owner, KinematicBody2D, Player).into_script();
		player_ref.map_mut(|player| {
			player.set_bullet_available(true);
		}).unwrap();

		get_node!(owner, Sprite, "Sprite").unwrap().hide();
		get_node!(owner, Particles2D, "PartsIdle").unwrap().set_emitting(false);
		get_node!(owner, Particles2D, "PartsPickUp").unwrap().set_emitting(true);
		get_node!(owner, Timer, "TimerDestroy").unwrap().start(0.0);
		self.can_pick_up = false;
		self.can_hit = false;
		owner.emit_signal("picked_up".into(), &[]);
	}

	#[export]
	pub unsafe fn _on_TimerPickUp_timeout(&mut self, owner: gd::RigidBody2D) {
		self.can_pick_up = true;
		if self.in_area {
			self.pick_up(owner);
		}
	}

	#[export]
	pub unsafe fn _on_PlayerBullet_body_entered(&mut self, _owner: gd::RigidBody2D, body: gd::Node) {
		if self.can_hit {
			if body.is_in_group("Enemy".into()) {
				let enemy_ref = get_instance_ref!(Enemy, body, KinematicBody2D);
				if !enemy_ref.into_script().map(|enemy| { enemy.is_healed() }).unwrap() {
					let enemy_ref2 = get_instance_ref!(Enemy, body, KinematicBody2D);
					enemy_ref2.map_mut(|enemy, owner| { enemy.hit(owner); }).unwrap();
				}
			}
			
			if body.is_in_group("EnemyBoss".into()) {
				let enemy_ref = get_instance_ref!(DemonKing, body, KinematicBody2D);
				if !enemy_ref.into_script().map(|enemy| { enemy.is_healed() }).unwrap() {
					let enemy_ref2 = get_instance_ref!(DemonKing, body, KinematicBody2D);
					enemy_ref2.map_mut(|enemy, owner| { enemy.hit(owner); }).unwrap();
				}
			}

			if body.is_in_group("Fountain".into()) {
				let fountain_ref = get_instance_ref!(Fountain, body, StaticBody2D);
				if !fountain_ref.into_script().map(|fount| { fount.is_purified() }).unwrap() {
					let fountain_ref2 = get_instance_ref!(Fountain, body, StaticBody2D);
					fountain_ref2.map_mut(|fount, owner| { fount.purify(owner, false); }).unwrap()
				}
			}
		}
	}

	#[export]
	pub unsafe fn _on_TimerStopHitting_timeout(&mut self, _owner: gd::RigidBody2D) {
		self.can_hit = false;
	}
}
