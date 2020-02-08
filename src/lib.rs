use gdnative::*;
use init::InitHandle;

//mod my_singleton;
//mod my_node2d;
mod player;
mod player_bullet;
mod macros;

fn init(handle: InitHandle) {
    //handle.add_class::<my_singleton::MySingleton>();
	//handle.add_class::<my_node2d::MyNode2D>();
	handle.add_class::<player::Player>();
	handle.add_class::<player_bullet::PlayerBullet>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
