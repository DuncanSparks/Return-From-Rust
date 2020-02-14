// WorldText.rs

use gdnative as gd;
use gd::init::property;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use crate::*;


#[derive(gd::NativeClass)]
#[inherit(gd::RichTextLabel)]
#[register_with(Self::register_properties)]
pub struct WorldText {
	text_to_display: GodotString,
	display: bool
}


#[methods]
impl WorldText {
	fn _init(_owner: gd::RichTextLabel) -> WorldText {
		WorldText {
			text_to_display: GodotString::new(),
			display: false
		}
	}

	fn register_properties(builder: &gd::init::ClassBuilder<Self>) {
		builder.add_property::<GodotString>("text_to_display")
		.with_default(GodotString::new())
		.with_hint(property::StringHint::Multiline)
		.with_setter(|this: &mut Self, _owner: RichTextLabel,  v| this.text_to_display = v)
		.with_getter(|this: &Self, _owner: RichTextLabel| this.text_to_display.new_ref())
		.done();
	}

	#[export]
	pub unsafe fn _ready(&self, mut owner: RichTextLabel) {
		owner.set_bbcode(format!("{}{}", "[center][wave amp=20 freq=3.5]", self.text_to_display.to_string()).into());
		owner.set_visible_characters(0);
	}

	#[export]
	pub unsafe fn _process(&self, mut owner: RichTextLabel, _delta: f64) {
		if self.display {
			let c = owner.get_visible_characters();
			owner.set_visible_characters(c + 1);
		}
	}

	#[export]
	pub fn set_display(&mut self, _owner: RichTextLabel, value: bool) {
		self.display = value;
	}
}
