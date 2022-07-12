#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*; //导出，外部可以调用
pub use traits::StorageInterface;

pub mod traits; //引入

#[frame_support::pallet]
pub mod pallet {

	//1.引入外部依赖，可以引入其它的依赖
	use super::*;
	use frame_support::{
		dispatch::{fmt::Debug, Codec},
		pallet_prelude::*,
		sp_runtime::traits::AtLeast32BitUnsigned,
	};
	// use frame_support::transactional;
	use frame_system::pallet_prelude::*;

	//2.声明pallet,可以理解为对象占位符号，固定写法
	#[pallet::pallet] //属性宏
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_); //T实际上就是runtime本身

	//3.定义trait，该trait必须继承frame_system::Config，同时可以在里面定义关联类型和类型约束
	#[pallet::config]
	pub trait Config: frame_system::Config {
		//步骤一：定义关联类型及其trait约束
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		//type StudentNumber:Get<u32>

		//定义类型，并指定约束
		// type StudentNumberType: Parameter
		// 	+ Member
		// 	+ AtLeast32BitUnsigned
		// 	+ Codec
		// 	+ Copy
		// 	+ Debug
		// 	+ Default
		// 	+ MaxEncodedLen
		// 	+ MaybeSerializeDeserialize;

		// type StudentNameType: Parameter
		// 	+ Member
		// 	+ AtLeast32BitUnsigned
		// 	+ Codec
		// 	+ Copy
		// 	+ Default
		// 	+ From<u128>
		// 	+ Into<u128>
		// 	+ MaxEncodedLen
		// 	+ MaybeSerializeDeserialize
		// 	+ Debug;

		type Value: Member //定义关联类型
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Codec
			+ From<u32>
			+ Into<u32>
			+ Copy
			+ Debug
			+ Default
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize;
	}

	//4.存储，定义变量存放地方
	//#[pallet::storage]
	//定义链上存储，有点同开辟存储空间
	//步骤二：定义存储，有四种：Storage Value、 Storage Map、Storage Double Map、Storage N Map
	//pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>;
	// //前面两项是默认，后面两项是k,v
	//#[pallet::getter(fn my_class)]
	//#[pallet::getter(fn //getter函数，与字段同名，只返回字段中的值，可以把字段变为私有，
	//#[pallet::getter(fn 然后通过api访问
	//pub type Class<T: Config> = StorageValue<_, u32>;
	// //存储名：Class，存储类型：StorageValue，只存一个值（任何类型），默认要实现约束Config

	//#[pallet::storage]
	//#[pallet::getter(fn set_flag)]
	//pub type HasSetFlag<T: Config> = StorageValue<_, Option<bool>>;

	//#[pallet::storage]
	//#[pallet::getter(fn student_info)] //ValueQuery:默认返回
	// pub type StudentInfo<T: Config> =
	// 	StorageMap<_, Blake2_128Concat, T::StudentNumberType, T::StudentNameType, ValueQuery>;
	//第二种存储类型，Map，k:StudentNumber,v:StudentName

	// #[pallet::storage]
	// #[pallet::getter(fn dorm_info)]
	// pub type DormInfo<T: Config> = StorageDoubleMap<
	// 	_,
	// 	Blake2_128Concat,
	// 	u32, //dorm number
	// 	Blake2_128Concat,
	// 	u32, //bed number
	// 	u32, //student number
	// 	ValueQuery,
	// >; //第三种存储类型，Double Map,k1,k2,v；//Blake2_128Concat为k的哈希

	#[pallet::storage]
	#[pallet::getter(fn my_value)] //ValueQuery:默认返回，T::Value 代表当前pallet的Value
	pub type MyValue<T: Config> = StorageValue<_, T::Value, ValueQuery>;

	//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] //生成发出事件的函数
														 //步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		//ClassSet(u32), //班级
		//StudentInfoSet(T::StudentNumberType, T::StudentNameType), //学生信息
		//DormInfoSet(u32, u32, u32), //寝室信息
		//SetParam(u32),
		FunctionCallSuccess,
	}

	//6.错误
	//步骤五：处理错误
	//和event类似，但error是当调度函数发生错误时发出的事件
	#[pallet::error]
	pub enum Error<T> {
		//ClassSetDuplicate,
		//NumberTooSmallThan100,
		//StudentInfoSetDuplicate,
		//DormInfoSetDuplicate,
		//FlagExisted,
		//FunctionCallFailed,
	}
	//错误定义和Event类似
	//步骤六：使用钩子
	//例如在某两个步骤之间打印日志
	//fn offchain_worker(_n,BlockNumber) {...}

	//7.钩子，如一些固定的动作
	// #[pallet::hooks]
	// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	// 	//实现两个函数
	// 	fn on_finalize(n: BlockNumberFor<T>) {
	// 		log::info!(target: "use-hooks","------- on_finalize,block number is {:?}",n);
	// 	}
	// 	fn on_initialize(n: BlockNumberFor<T>) -> Weight {
	// 		log::info!(target: "use-hooks","+++++++ on_initialize,block number is {:?}",n);
	// 		0
	// 	}
	// }

	//8.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//步骤四：编写调度函数;调度函数都是围绕着存储类型的方法展开的
		// #[pallet::weight(0)] //引入权重宏
		// pub fn create_claim(
		// 	//存储的k,v 参数以及 origin，origin 是必须的，指的是调用者
		// 	origin: OriginFor<T>,
		// 	id: u32,
		// 	claim: u128,
		// ) -> DispatchResultWithPostInfo {
		// 	//1.判断，使用？代替match处理枚举结，如果出错会提前返回，否则继续执行
		// 	ensure_signed(origin)?;
		// 	//2.执行存储，调用实例的insert方法
		// 	Proofs::<T>::insert(&id, &claim);
		// 	//3.产生通知事件
		// 	Self::deposit_event(Event::ClaimCreated(id, claim));
		// 	//4.返回单元组类型
		// 	Ok(().into())
		// }

		// #[transactional] //自动回滚
		// //每一个存储变量对应一个调度函数
		// #[pallet::weight(0)] //所有调度函数都需要，操作成本，权重可以动态变化根据条件
		// 		 //函数名称与存储名称在语义上保持统一，函数是对存储的操作，函数的结果使用Result枚举处理
		// pub fn set_class_info(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
		// 	//1.判断条件: 签名、是否是root账户
		// 	ensure_root(origin)?; //只有root账户才能操作

		// 	// //1.先判断
		// 	// if Class::<T>::exists() {
		// 	// 	return Err(Error::<T>::ClassSetDuplicate.into());
		// 	// }

		// 	//2.操作
		// 	//操作先于判断之前执行，但是由于用了transactional，所以判断失败后，状态会回滚
		// 	Class::<T>::put(class); //StorageValue 使用put方法存储值，其他方法可以去官方文档查看

		// 	if HasSetFlag::<T>::exists() {
		// 		return Err(Error::<T>::FlagExisted.into());
		// 	}

		// 	HasSetFlag::<T>::put(Some(true));

		// 	// if class <= 100u32 {
		// 	// 	return Err(Error::<T>::NumberTooSmallThan100.into());
		// 	// }

		// 	// let _c = Self::my_class(); //调用getter函数

		// 	//3.发出事件通知
		// 	Self::deposit_event(Event::ClassSet(class));

		// 	Ok(().into()) //把错误装箱
		// }

		// #[transactional]
		// #[pallet::weight(0)]
		// pub fn set_param_bigger_than_100(origin: OriginFor<T>, param: u32) -> DispatchResult {
		// 	ensure_root(origin)?;

		// 	if param <= 100 {
		// 		return Err(Error::<T>::NumberTooSmallThan100.into());
		// 	}

		// 	Class::<T>::put(param);
		// 	//3、发出事件
		// 	Self::deposit_event(Event::SetParam(param));
		// 	//添加log
		// 	log::info!(target: "use-hooks", "set param bigger than 100");

		// 	Ok(().into())
		// }

		// #[pallet::weight(0)]
		// pub fn set_student_info(
		// 	origin: OriginFor<T>,
		// 	student_number: T::StudentNumberType,
		// 	student_name: T::StudentNameType,
		// ) -> DispatchResultWithPostInfo {
		// 	ensure_root(origin)?;

		// 	//先判断
		// 	if StudentInfo::<T>::contains_key(student_number) {
		// 		return Err(Error::<T>::StudentInfoSetDuplicate.into());
		// 	}

		// 	StudentInfo::<T>::insert(&student_number, &student_name);

		// 	//发出事件通知
		// 	Self::deposit_event(Event::StudentInfoSet(student_number, student_name));

		// 	Ok(().into())
		// }
		// #[pallet::weight(0)]
		// pub fn set_dorm_info(
		// 	origin: OriginFor<T>,
		// 	dorm_number: u32,
		// 	bed_number: u32,
		// 	student_number: u32,
		// ) -> DispatchResultWithPostInfo {
		// 	ensure_root(origin)?;

		// 	//先判断
		// 	if DormInfo::<T>::contains_key(dorm_number, bed_number) {
		// 		return Err(Error::<T>::DormInfoSetDuplicate.into());
		// 	}

		// 	DormInfo::<T>::insert(&dorm_number, &bed_number, &student_number);

		// 	//发出事件通知
		// 	Self::deposit_event(Event::DormInfoSet(dorm_number, bed_number, student_number));

		// 	Ok(().into())
		// }

		//这个调度函数
		#[pallet::weight(0)]
		pub fn my_function(origin: OriginFor<T>, value: u128) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			log::info!(target:"storage provider","my function!");

			Self::deposit_event(Event::FunctionCallSuccess);

			Ok(().into())
		}
	}
}

//给类型实现trait
impl<T: Config> StorageInterface for Pallet<T> {
	type Value = T::Value;
	//在trait中实现函数而不是在当前pallet的调度模块中实现
	fn get_param() -> Self::Value {
		MyValue::<T>::get()
	}
	fn set_param(v: Self::Value) {
		MyValue::<T>::put(v);
	}
}
