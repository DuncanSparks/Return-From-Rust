// LoadingZone.rs

use gdnative as gd;
use gd::init::property;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use player::Player;
use player::Direction;
use controller::Controller;

use crate::*;


#[derive(gd::NativeClass)]
#[inherit(gd::Area2D)]
#[register_with(Self::register_properties)]
pub struct LoadingZone {
	target_scene: GodotString,
	direction: u8
}


#[methods]
impl LoadingZone {
	fn _init(_owner: gd::Area2D) -> LoadingZone {
		LoadingZone {
			target_scene: GodotString::new(),
			direction: 0
		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_property::<GodotString>("target_scene")
		.with_default(GodotString::new())
		.with_setter(|this: &mut Self, _owner: Area2D,  v| this.target_scene = v)
		.with_hint(property::StringHint::File(property::EnumHint::new(vec!["*.tscn".into()])))
		.done();

		builder.add_property::<u8>("direction")
		.with_default(0)
		.with_setter(|this: &mut Self, _owner: Area2D,  v| this.direction = v)
		.with_hint(property::IntHint::Enum(property::EnumHint::new(vec!["Up".into(), "Down".into(), "Left".into(), "Right".into()])))
		.done();
	}

	#[export]
	pub unsafe fn _on_LoadingZone_body_entered(&mut self, owner: Area2D, body: Node) {
		if body.is_in_group("Player".into()) {
			let player_ref = get_instance_ref!(Player, body, KinematicBody2D);
			godot_print!("MARK 1");
			if player_ref.into_script().map(|player| { !player.is_loading() }).unwrap() {
				let player_ref_2 = get_instance_ref!(Player, body, KinematicBody2D).into_script();
				godot_print!("MARK 2");
				player_ref_2.map_mut(|player| { player.set_loading(true); }).unwrap();
				player_ref_2.map_mut(|player| { player.set_face(Direction::from_u8(self.direction)); }).unwrap();

				godot_print!("MARK 3");

				let mut player_ref_3 = owner.get_node(NodePath::from(format!("{}{}", "/root/", "Player")).new_ref()).unwrap().cast::<KinematicBody2D>().unwrap();
				let pos = player_ref_3.get_position();
				godot_print!("MARK 4");
				match self.direction {
					0 => player_ref_3.set_position(Vector2::new(pos.x, 160.0)),
					1 => player_ref_3.set_position(Vector2::new(pos.x, 20.0)),
					2 => player_ref_3.set_position(Vector2::new(300.0, pos.y)),
					3 => player_ref_3.set_position(Vector2::new(20.0, pos.y)),
					_ => player_ref_3.set_position(Vector2::new(pos.x, 160.0))
				}

				godot_print!("MARK 5");

				owner.get_tree().unwrap().change_scene(self.target_scene.clone()).unwrap();

				godot_print!("MARK 6");

				player_ref_2.map_mut(|player| { player.set_bullet_available(true); }).unwrap();

				godot_print!("MARK 7");

				get_singleton!(owner, Node, Controller).map_mut(|contr, owner| { contr.after_load(owner); }).unwrap();
				godot_print!("MARK 9");
			}
		}
	}
}
