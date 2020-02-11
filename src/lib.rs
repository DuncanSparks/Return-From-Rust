#![allow(non_snake_case)]

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
mod world_text;
mod room;
mod demon_king;
mod boss_bullet;
mod game_over;

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
	handle.add_class::<world_text::WorldText>();
	handle.add_class::<room::Room>();
	handle.add_class::<demon_king::DemonKing>();
	handle.add_class::<game_over::GameOver>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
