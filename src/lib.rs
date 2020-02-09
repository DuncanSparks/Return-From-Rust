use gdnative::*;
use init::InitHandle;

mod player;
mod player_bullet;
mod controller;
mod macros;
mod enemy;
mod bullet;
mod ground_attack;
mod fountain;
mod loading_zone;
mod start;
mod title;

fn init(handle: InitHandle) {
	handle.add_class::<player::Player>();
	handle.add_class::<player_bullet::PlayerBullet>();
	handle.add_class::<controller::Controller>();
	handle.add_class::<enemy::Enemy>();
	handle.add_class::<bullet::Bullet>();
	handle.add_class::<ground_attack::GroundAttack>();
	handle.add_class::<fountain::Fountain>();
	handle.add_class::<loading_zone::LoadingZone>();
	handle.add_class::<start::Start>();
	handle.add_class::<title::Title>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
