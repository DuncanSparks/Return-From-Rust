// Cursor.rs

use gdnative as gd;
use gd::{methods, godot_wrap_method, godot_wrap_method_inner, godot_error, godot_wrap_method_parameter_count};

use crate::*;


#[derive(gd::NativeClass)]
#[inherit(gd::CanvasLayer)]
#[user_data(gd::user_data::LocalCellData<Cursor>)]
#[register_with(Self::register_properties)]
pub struct Cursor {
    curs: Option<Sprite>
}


#[methods]
impl Cursor {
	fn _init(_owner: gd::CanvasLayer) -> Cursor {
		Cursor {
            curs: None
		}
	}

	fn register_properties(_builder: &gd::init::ClassBuilder<Self>) {
    }
    
    #[export]
    pub unsafe fn _ready(&mut self, owner: CanvasLayer) {
        self.curs = get_node!(owner, Sprite, "Cursor");
    }

	#[export]
	pub unsafe fn _process(&mut self, owner: CanvasLayer, _delta: f64) {
        self.curs.unwrap().set_position(owner.get_viewport().unwrap().get_mouse_position());
    }
}
