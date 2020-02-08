#[macro_export]
macro_rules! load {
	($p:expr) => {
		ResourceLoader::godot_singleton().load($p.into(), "".into(), false).unwrap().cast::<PackedScene>()
	};
}

#[macro_export]
macro_rules! move_and_slide_default {
	($o:expr, $v:expr) => {
		$o.move_and_slide($v, Vector2::zero(), false, 4, 0.785398, true)
	};
}

#[macro_export]
macro_rules! get_node {
	($o:expr, $t:ty, $n:expr) => {
		($o.get_node($n.into()).unwrap()).cast::<$t>()
	};
}

#[macro_export]
macro_rules! get_instance_ref {
	($t1:ty, $p:expr, $t2:ty) => {
		Instance::<$t1>::try_from_unsafe_base($p.cast::<$t2>().unwrap()).unwrap()
	};
}

#[macro_export]
macro_rules! deallocate {
	($r:expr) => {
		$r.as_ref().unwrap().free()
	};
}
