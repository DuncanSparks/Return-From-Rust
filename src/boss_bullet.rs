// BossBullet.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};
use gd::user_data::*;

use player::Player;

use crate::*;


#[derive(gd::NativeClass)]
#[inherit(gd::RigidBody2D)]
#[user_data(gd::user_data::LocalCellData<BossBullet>)]
#[register_with(Self::register_properties)]
pub struct BossBullet {
	stopped: bool,

	can_pick_up: bool,
	can_hit: bool,
}


#[methods]
impl BossBullet {
	const SPEED: f32 = 250.0;

	fn _init(_owner: gd::RigidBody2D) -> BossBullet {
		BossBullet {
			stopped: false,
			can_pick_up: false,
			can_hit: true
		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_property::<bool>("stopped")
		.with_default(false)
		.with_setter(|this: &mut Self, _owner: RigidBody2D,  v| this.stopped = v)
		.with_getter(|this: &Self, _owner: RigidBody2D| this.stopped)
		.done();

		/*builder.add_signal(init::Signal {
			name: "picked_up",
			args: &[]
		});*/

	}

	#[export]
	pub unsafe fn _ready(&mut self, mut owner: RigidBody2D) {
		if !self.stopped {
			let ang = Angle::radians(owner.get_global_rotation() as f32);
			let ang_deg = ang.positive().to_degrees() - 90.0;
			owner.set_linear_velocity(Vector2::new(BossBullet::SPEED, 0.0).rotated(-Angle::radians(ang_deg.to_radians())));
		}
		else {
			owner.set_angular_velocity(0.0);
		}
	}

	#[export]
	pub unsafe fn _process(&mut self, mut owner: RigidBody2D, _delta: f64) {
		let y = owner.get_position().y as i64;
		owner.set_z_index(y);
	}

	#[export]
	pub unsafe fn _on_AreaPickUp_body_entered(&mut self, mut owner: RigidBody2D, body: Node) {
		if body.is_in_group("Player".into()) && self.can_pick_up {
			get_node!(owner, AudioStreamPlayer, "SoundPickUp").unwrap().play(0.0);

			let player_ref = get_instance_ref!(Player, body, KinematicBody2D).into_script();
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
	}

	#[export]
	pub unsafe fn _on_TimerPickUp_timeout(&mut self, _owner: RigidBody2D) {
		self.can_pick_up = true;
	}

	#[export]
	pub unsafe fn _on_BossBullet_body_entered(&mut self, _owner: RigidBody2D, body: Node) {
		if self.can_hit {
			if body.is_in_group("Player".into()) {
				let player_ref = get_instance_ref!(Player, body, KinematicBody2D);
				if player_ref.into_script().map(|player| { !player.is_in_iframes() }).unwrap() {
					let player_ref2 = get_instance_ref!(Player, body, KinematicBody2D);
					player_ref2.map_mut(|player, owner| { player.damage(owner, 2); }).unwrap();
				}
			}
		}
	}

	#[export]
	pub unsafe fn _on_TimerStopHitting_timeout(&mut self, _owner: RigidBody2D) {
		self.can_hit = false;
	}

	#[export]
	pub unsafe fn _on_TimerExplode_timeout(&self, owner: RigidBody2D) {
		get_node!(owner, AnimationPlayer, "AnimationPlayer2").unwrap().play("Explode".into(), -1.0, 1.0, false);
	}

	#[export]
	pub unsafe fn _on_AnimationPlayer2_animation_finished(&self, mut owner: RigidBody2D, _anim_name: GodotString) {
		owner.queue_free();
	}
}
