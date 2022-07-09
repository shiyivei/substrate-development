#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*; //导出，外部可以调用
#[frame_support::pallet]
pub mod pallet {

	//1.引入外部依赖，可以引入其它的依赖
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	//2.声明pallet,可以理解为对象占位符号，固定写法
	#[pallet::pallet] //属性宏
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	//3.定义trait，该trait必须继承frame_system::Config，同时可以在里面定义关联类型和类型约束
	#[pallet::config]
	pub trait Config: frame_system::Config {
		//步骤一：定义关联类型及其trait约束
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	//4.存储，定义变量存放地方
	#[pallet::storage] //定义链上存储，有点同开辟存储空间
				   //步骤二：定义存储，有四种：Storage Value、 Storage Map、Storage Double Map、Storage N Map
	pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>; //前面两项是默认，后面两项是k,v

	//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	//步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		ClaimCreated(u32, u128), //元组
	}

	//6.钩子，如一些固定的动作
	//#[pallet::hooks]
	//impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

	//7.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//步骤四：编写调度函数
		#[pallet::weight(0)] //引入权重宏
		pub fn create_claim(
			//存储的k,v 参数以及 origin，origin 是必须的，指的是调用者
			origin: OriginFor<T>,
			id: u32,
			claim: u128,
		) -> DispatchResultWithPostInfo {
			//1.判断，使用？代替match处理枚举结，如果出错会提前返回，否则继续执行
			ensure_signed(origin)?;
			//2.执行存储，调用实例的insert方法
			Proofs::<T>::insert(&id, &claim);
			//3.产生通知事件
			Self::deposit_event(Event::ClaimCreated(id, claim));
			//4.返回单元组类型
			Ok(().into())
		}
	}

	//步骤五：处理错误
	//错误定义和Event类似
	//步骤六：使用钩子
	//例如在某两个步骤之间打印日志
	//fn offchain_worker(_n,BlockNumber) {...}
}
