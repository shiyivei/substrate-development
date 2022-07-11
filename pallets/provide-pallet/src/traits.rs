//定一个一个trait，里面封装了在一个存储类型上执行操作的方法

pub trait StorageInterface {
	type Value;
	fn get_param() -> Self::Value;
	fn set_param(v: Self::Value);
}
