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
macro_rules! get_singleton {
	($o:expr, $t:ty, $n:ty) => {
		Instance::<$n>::try_from_unsafe_base($o.get_node(NodePath::from(format!("{}{}", "/root/", stringify!($n))).new_ref()).unwrap().cast::<$t>().unwrap()).unwrap();
	};
}

#[macro_export]
macro_rules! rand_range {
	($o:expr, $f:expr, $t:expr) => {
		get_singleton!($o, Node, Controller).into_script().map_mut(|contr| { contr.rand_range($f, $t) }).unwrap()
	};
}
