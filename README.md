# 1 Substrate 简介

## 1.1 开始一条本地链

**1.下载repo**

```
git clone https://github.com/substrate-developer-hub/substrate-node-template 
```

**2.切换分支**

```
cd substrate-node-template && git checkout latest
```

**3.优化编译**

```
cargo build --release
```

**4.启动节点**

```
./target/release/node-template --dev // --dev 代表开发者模式，强行结束后会删除所有数据
```

## 1.2 Substrate 基础知识

**1.Node内容**

链的基础内容，基本功能的实现（网络、rpc...等）

**2.实现逻辑**

pallet，也是我们重点实现业务逻辑的方式

**3.Runtime**

逻辑集合

**4.三种开发方式：**

1. 直接使用substrate node
2. 使用substrate frame构建运行时
3. 使用core开发

# 2 编写Pallet的Rust前置知识

```
#[pallet::config] 
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}
```

# 3 编写简单的Pallet

**1.拷贝**

```
cp -r template simple-pallet
```

**2.修改pallet名字**

```
[package]
name = "pallet-simple-pallet"
```

**3.删除多余文件**

```
rm -rf benchmarking.rs mock.rs tests.rs
```

```
再删掉lib.rs中的所有内容
```

**4.编写pallet的7部分**

```
#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*; //导出，外部可以调用
#[frame_support::pallet]
pub mod pallet {

	//1.引入外部依赖，可以引入其它的依赖
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	//2.声明pallet,可以理解为对象占位符号，固定写法
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	//3.定义trait，该trait必须继承frame_system::Config，同时可以在里面定义关联类型和类型约束
	#[pallet::config]
	pub trait Config: frame_system::Config {
		//步骤一：定义关联类型及其trait约束
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	//4.存储，定义变量存放地方
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type MyStorage<T: Config> = StorageValue<_, u32>;
	//步骤二：定义存储，有四种：Storage Value、 Storage Map、Storage Double Map、Storage N Map
	pub type Proofs<T: Config> = StorageMap<_, Blake2_128Contact, u32, u128>; //前面两项是默认，后面两项是k,v

	//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	//步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		ClaimCreated(u32, u128),
	}

	//6.钩子，如一些固定的动作
	//#[pallet::hooks]
	//impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

	//7.调度函数
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
	//步骤六：使用钩子
}
```

**5.在runtime文件中加入依赖**

**cargo.toml**

```
# Local Dependencies
pallet-template = { version = "4.0.0-dev", default-features = false, path = "../pallets/template" }
pallet-simple-pallet = { version = "4.0.0-dev", default-features = false, path = "../pallets/simple-pallet"}
```

```
"pallet-simple-pallet/std",
```

**lib.rs**

```
//配置
impl pallet_simple_pallet::Config for Runtime {
	type Event = Event;
}
```

```
SimplePallet:pallet_simple_pallet, //构建
```

**6.编译并运行**

```
cargo build --release
```

```
./target/release/node-template --tmp --dev
```

# 4 pallet 的组成再研究

```
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
```

# 5 Storage

## 5.1 初始化&配置pallet

更像一个应用层的概念，可以查看定义storeage的宏

**1.复制一个pallet模版**

```
cp -r simple-pallet use-storage
```

**2.改名字**

```
version = "0.0.1"
description = "example for storage"
authors = ["shiyivei"]
```

在runtime cargo.toml中添加依赖

```
pallet-use-storage = { version = "0.0.1", default-features = false, path = "../pallets/use-storage"}
```

添加features

```
"pallet-use-storage/std"
```

**3.在runtime中实现和添加pallet**

```
//实现pallet
impl pallet_use_storage::Config for Runtime {}
```

```
UseStorage:pallet_use_storage, //添加实现的pallet
```

**4.编译**

确保所有流程都已经操作完毕

```
cargo build --release
```

格式化代码

```
cargo +nightly fmt
```

## 5.2 编写pallet

事实上，我们只编写了storage和调度函数

```
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
		//type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	//4.存储，定义变量存放地方
	#[pallet::storage]
	//定义链上存储，有点同开辟存储空间
	//步骤二：定义存储，有四种：Storage Value、 Storage Map、Storage Double Map、Storage N Map
	//pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>; //前面两项是默认，后面两项是k,v
	#[pallet::getter(fn my_class)] //getter函数，与字段同名，只返回字段中的值，可以把字段变为私有，然后通过api访问
	pub type Class<T: Config> = StorageValue<_, u32>; //存储名：Class，存储类型：StorageValue，只存一个值（任何类型），默认要实现约束Config

	#[pallet::storage]
	#[pallet::getter(fn student_info)] //ValueQuery:默认返回
	pub type StudentInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>; //第二种存储类型，Map，k:StudentNumber,v:StudentName

	#[pallet::storage]
	#[pallet::getter(fn dorm_info)]
	pub type DormInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32, //dorm number
		Blake2_128Concat,
		u32, //bed number
		u32, //student number
		ValueQuery,
	>; //第三种存储类型，Double Map,k1,k2,v；//Blake2_128Concat为k的哈希

	//5.链上事件的通知
	// #[pallet::event]
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	//步骤三：操作执行成功后通知用户
	// pub enum Event<T: Config> {
	// 	//ClaimCreated(u32, u128), //元组
	// }

	//6.钩子，如一些固定的动作
	//#[pallet::hooks]
	//impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

	//7.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//步骤四：编写调度函数
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
		//每一个存储变量对应一个调度函数
		#[pallet::weight(0)] //所有调度函数都需要，操作成本
				 //函数名称与存储名称在语义上保持统一，函数是对存储的操作，函数的结果使用Result枚举处理
		pub fn set_class_info(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			ensure_root(origin)?; //只有root账户才能操作
			Class::<T>::put(class); //StorageValue 使用put方法存储值，其他方法可以去官方文档查看

			//省略了通知

			Ok(().into()) //把错误装箱
		}

		#[pallet::weight(0)]
		pub fn set_student_info(
			origin: OriginFor<T>,
			student_number: u32,
			student_name: u128,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			StudentInfo::<T>::insert(&student_number, &student_name);

			Ok(().into())
		}
		#[pallet::weight(0)]
		pub fn set_dorm_info(
			origin: OriginFor<T>,
			dorm_number: u32,
			bed_number: u32,
			student_number: u32,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			DormInfo::<T>::insert(&dorm_number, &bed_number, &student_number);

			Ok(().into())
		}
	}

	//步骤五：处理错误
	//错误定义和Event类似
	//步骤六：使用钩子
	//例如在某两个步骤之间打印日志
	//fn offchain_worker(_n,BlockNumber) {...}
}
```

## **5.3 编译并运行**

```
cargo build --release
```

```
./target/release/node-template --tmp --dev
```

然后在前端使用sudo权限操作

# 6 Error和Event

## 6.1 初始化&配置pallet

**1.复制一个pallet模版**

```
cp -r simple-pallet use-storage
```

**2.改名字**

```
[package]
name = "pallet-events-errors"
version = "0.0.1"
description = "example for events and errors"
```

在runtime cargo.toml中添加依赖

```
pallet-events-errors = { version = "0.0.1", default-features = false,path = "../pallets/events-errors"}
```

添加features

```
"pallet-events-errors/std"
```

## 6.2 Events

### 6.2.1 编写pallet

增加事件类型

```
//3.定义trait，该trait必须继承frame_system::Config，同时可以在里面定义关联类型和类型约束
	#[pallet::config]
	pub trait Config: frame_system::Config {
		//步骤一：定义关联类型及其trait约束
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}
```

定义事件

```
//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] //生成发出事件的函数
											  //步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		ClassSet(u32),              //班级
		StudentInfoSet(u32, u128),  //学生信息
		DormInfoSet(u32, u32, u32), //寝室信息
	}
```

发出通知

```
Self::deposit_event(Event::ClassSet(class));
Self::deposit_event(Event::StudentInfoSet(student_number, student_name));
Self::deposit_event(Event::DormInfoSet(dorm_number, bed_number, student_number));
```

pallet完整代码

```
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
	#[pallet::storage]
	//定义链上存储，有点同开辟存储空间
	//步骤二：定义存储，有四种：Storage Value、 Storage Map、Storage Double Map、Storage N Map
	//pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>; //前面两项是默认，后面两项是k,v
	#[pallet::getter(fn my_class)] //getter函数，与字段同名，只返回字段中的值，可以把字段变为私有，然后通过api访问
	pub type Class<T: Config> = StorageValue<_, u32>; //存储名：Class，存储类型：StorageValue，只存一个值（任何类型），默认要实现约束Config

	#[pallet::storage]
	#[pallet::getter(fn student_info)] //ValueQuery:默认返回
	pub type StudentInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>; //第二种存储类型，Map，k:StudentNumber,v:StudentName

	#[pallet::storage]
	#[pallet::getter(fn dorm_info)]
	pub type DormInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32, //dorm number
		Blake2_128Concat,
		u32, //bed number
		u32, //student number
		ValueQuery,
	>; //第三种存储类型，Double Map,k1,k2,v；//Blake2_128Concat为k的哈希

	//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] //生成发出事件的函数
											  //步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		ClassSet(u32),              //班级
		StudentInfoSet(u32, u128),  //学生信息
		DormInfoSet(u32, u32, u32), //寝室信息
	}

	//6.钩子，如一些固定的动作
	//#[pallet::hooks]
	//impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

	//7.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//步骤四：编写调度函数
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
		//每一个存储变量对应一个调度函数
		#[pallet::weight(0)] //所有调度函数都需要，操作成本
				 //函数名称与存储名称在语义上保持统一，函数是对存储的操作，函数的结果使用Result枚举处理
		pub fn set_class_info(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			ensure_root(origin)?; //只有root账户才能操作
			Class::<T>::put(class); //StorageValue 使用put方法存储值，其他方法可以去官方文档查看

			// let _c = Self::my_class(); //调用getter函数
			//发出事件通知
			Self::deposit_event(Event::ClassSet(class));

			Ok(().into()) //把错误装箱
		}

		#[pallet::weight(0)]
		pub fn set_student_info(
			origin: OriginFor<T>,
			student_number: u32,
			student_name: u128,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			StudentInfo::<T>::insert(&student_number, &student_name);

			//发出事件通知
			Self::deposit_event(Event::StudentInfoSet(student_number, student_name));

			Ok(().into())
		}
		#[pallet::weight(0)]
		pub fn set_dorm_info(
			origin: OriginFor<T>,
			dorm_number: u32,
			bed_number: u32,
			student_number: u32,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			DormInfo::<T>::insert(&dorm_number, &bed_number, &student_number);

			//发出事件通知
			Self::deposit_event(Event::DormInfoSet(dorm_number, bed_number, student_number));

			Ok(().into())
		}
	}

	//步骤五：处理错误
	//错误定义和Event类似
	//步骤六：使用钩子
	//例如在某两个步骤之间打印日志
	//fn offchain_worker(_n,BlockNumber) {...}
}
```

**3.在runtime中实现和添加pallet**

```
//实现pallet
impl pallet_events_errors::Config for Runtime {
	type Event = Event;
}
```

```
EventsErrors:pallet_events_errors, //添加实现的pallet
```

**4.编译**

确保所有流程都已经操作完毕

```
cargo build --release
```

格式化代码

```
cargo +nightly fmt
```

### **6.2.2 编译并运行**

```
cargo build --release
```

```
./target/release/node-template --tmp --dev
```

然后在前端使用sudo权限操作，可以看到此pallet当调度函数执行成功时，会发出事件

## 6.3 Errors

### 6.2.1 编写pallet

增加错误

```
//步骤五：处理错误
	//和event类似，但error是当调度函数发生错误时发出的事件
	#[pallet::error]
	pub enum Error<T> {
		ClassSetDuplicate,
		StudentInfoSetDuplicate,
		DormInfoSetDuplicate,
	}
```

指定错误条件

```
//先判断
			if Class::<T>::exists() {
				return Err(Error::<T>::ClassSetDuplicate.into());
			}
```

```
//先判断
			if StudentInfo::<T>::contains_key(student_number) {
				return Err(Error::<T>::StudentInfoSetDuplicate.into());
			}
```

```
//先判断
			if DormInfo::<T>::contains_key(dorm_number, bed_number) {
				return Err(Error::<T>::DormInfoSetDuplicate.into());
			}
```

pallet完整代码

```
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
	#[pallet::storage]
	//定义链上存储，有点同开辟存储空间
	//步骤二：定义存储，有四种：Storage Value、 Storage Map、Storage Double Map、Storage N Map
	//pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>; //前面两项是默认，后面两项是k,v
	#[pallet::getter(fn my_class)] //getter函数，与字段同名，只返回字段中的值，可以把字段变为私有，然后通过api访问
	pub type Class<T: Config> = StorageValue<_, u32>; //存储名：Class，存储类型：StorageValue，只存一个值（任何类型），默认要实现约束Config

	#[pallet::storage]
	#[pallet::getter(fn student_info)] //ValueQuery:默认返回
	pub type StudentInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>; //第二种存储类型，Map，k:StudentNumber,v:StudentName

	#[pallet::storage]
	#[pallet::getter(fn dorm_info)]
	pub type DormInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32, //dorm number
		Blake2_128Concat,
		u32, //bed number
		u32, //student number
		ValueQuery,
	>; //第三种存储类型，Double Map,k1,k2,v；//Blake2_128Concat为k的哈希

	//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] //生成发出事件的函数
											  //步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		ClassSet(u32),              //班级
		StudentInfoSet(u32, u128),  //学生信息
		DormInfoSet(u32, u32, u32), //寝室信息
	}

	//6.钩子，如一些固定的动作
	//#[pallet::hooks]
	//impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

	//7.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//步骤四：编写调度函数
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
		//每一个存储变量对应一个调度函数
		#[pallet::weight(0)] //所有调度函数都需要，操作成本
				 //函数名称与存储名称在语义上保持统一，函数是对存储的操作，函数的结果使用Result枚举处理
		pub fn set_class_info(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			ensure_root(origin)?; //只有root账户才能操作

			//先判断
			if Class::<T>::exists() {
				return Err(Error::<T>::ClassSetDuplicate.into());
			}
			Class::<T>::put(class); //StorageValue 使用put方法存储值，其他方法可以去官方文档查看

			// let _c = Self::my_class(); //调用getter函数
			//发出事件通知
			Self::deposit_event(Event::ClassSet(class));

			Ok(().into()) //把错误装箱
		}

		#[pallet::weight(0)]
		pub fn set_student_info(
			origin: OriginFor<T>,
			student_number: u32,
			student_name: u128,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			//先判断
			if StudentInfo::<T>::contains_key(student_number) {
				return Err(Error::<T>::StudentInfoSetDuplicate.into());
			}

			StudentInfo::<T>::insert(&student_number, &student_name);

			//发出事件通知
			Self::deposit_event(Event::StudentInfoSet(student_number, student_name));

			Ok(().into())
		}
		#[pallet::weight(0)]
		pub fn set_dorm_info(
			origin: OriginFor<T>,
			dorm_number: u32,
			bed_number: u32,
			student_number: u32,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			//先判断
			if DormInfo::<T>::contains_key(dorm_number, bed_number) {
				return Err(Error::<T>::DormInfoSetDuplicate.into());
			}

			DormInfo::<T>::insert(&dorm_number, &bed_number, &student_number);

			//发出事件通知
			Self::deposit_event(Event::DormInfoSet(dorm_number, bed_number, student_number));

			Ok(().into())
		}
	}

	//步骤五：处理错误
	//和event类似，但error是当调度函数发生错误时发出的事件
	#[pallet::error]
	pub enum Error<T> {
		ClassSetDuplicate,
		StudentInfoSetDuplicate,
		DormInfoSetDuplicate,
	}
	//错误定义和Event类似
	//步骤六：使用钩子
	//例如在某两个步骤之间打印日志
	//fn offchain_worker(_n,BlockNumber) {...}
}
```

**3.在runtime中实现和添加pallet**

```
//实现pallet
impl pallet_events_errors::Config for Runtime {
	type Event = Event;
}
```

```
EventsErrors:pallet_events_errors, //添加实现的pallet
```

**4.编译**

确保所有流程都已经操作完毕

```
cargo build --release
```

格式化代码

```
cargo +nightly fmt
```

### **6.2.2 编译并运行**

```
cargo build --release
```

```
./target/release/node-template --tmp --dev
```

然后在前端使用sudo权限操作，可以看到此pallet当调度函数执行成功时，会发出事件

# 7 调度函数

## 7.1 函数编写三步骤

有三种调度函数extrinsics,分别是Signed transactions和Unsigned transactions

签名交易需要支付交易费，预防垃圾信息的经济逻辑

### 7.1.1 初始化&配置pallet

更像一个应用层的概念，可以查看定义storeage的宏

**1.复制一个pallet模版**

```
cp -r events-errors call-function
```

**2.改名字**

```
[package]
name = "pallet-call-function"
version = "0.0.1"
description = "example for storage"
authors = ["shiyivei"]
```

在runtime cargo.toml中添加依赖

```
pallet-call-function = { version = "0.0.1", default-features = false, path = "../pallets/call-function"}
```

添加features

```
"pallet-call-function/std"
```

**3.在runtime中实现和添加pallet**

```
//实现pallet
impl pallet_call_function::Config for Runtime {
	type Event = Event;
}
```

```
CallFunction:pallet_call_function, //添加实现的pallet
```

**4.编译**

确保所有流程都已经操作完毕

```
cargo build --release
```

格式化代码

```
cargo +nightly fmt
```

### 7.1.2 编写pallet

事实上，我们只编写了storage和调度函数

```
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
	pub struct Pallet<T>(_); //T实际上就是runtime本身

	//3.定义trait，该trait必须继承frame_system::Config，同时可以在里面定义关联类型和类型约束
	#[pallet::config]
	pub trait Config: frame_system::Config {
		//步骤一：定义关联类型及其trait约束
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		//type StudentNumber:Get<u32>
	}

	//4.存储，定义变量存放地方
	#[pallet::storage]
	//定义链上存储，有点同开辟存储空间
	//步骤二：定义存储，有四种：Storage Value、 Storage Map、Storage Double Map、Storage N Map
	//pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>; //前面两项是默认，后面两项是k,v
	#[pallet::getter(fn my_class)] //getter函数，与字段同名，只返回字段中的值，可以把字段变为私有，然后通过api访问
	pub type Class<T: Config> = StorageValue<_, u32>; //存储名：Class，存储类型：StorageValue，只存一个值（任何类型），默认要实现约束Config

	#[pallet::storage]
	#[pallet::getter(fn student_info)] //ValueQuery:默认返回
	pub type StudentInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>; //第二种存储类型，Map，k:StudentNumber,v:StudentName

	#[pallet::storage]
	#[pallet::getter(fn dorm_info)]
	pub type DormInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32, //dorm number
		Blake2_128Concat,
		u32, //bed number
		u32, //student number
		ValueQuery,
	>; //第三种存储类型，Double Map,k1,k2,v；//Blake2_128Concat为k的哈希

	//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] //生成发出事件的函数
											  //步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		ClassSet(u32),              //班级
		StudentInfoSet(u32, u128),  //学生信息
		DormInfoSet(u32, u32, u32), //寝室信息
	}

	//6.钩子，如一些固定的动作
	//#[pallet::hooks]
	//impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

	//7.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
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
		//每一个存储变量对应一个调度函数
		#[pallet::weight(0)] //所有调度函数都需要，操作成本，权重可以动态变化根据条件
				 //函数名称与存储名称在语义上保持统一，函数是对存储的操作，函数的结果使用Result枚举处理
		pub fn set_class_info(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			//1.判断条件: 签名、是否是root账户
			ensure_root(origin)?; //只有root账户才能操作

			//1.先判断
			if Class::<T>::exists() {
				return Err(Error::<T>::ClassSetDuplicate.into());
			}

			//2.操作
			Class::<T>::put(class); //StorageValue 使用put方法存储值，其他方法可以去官方文档查看

			// let _c = Self::my_class(); //调用getter函数

			//3.发出事件通知
			Self::deposit_event(Event::ClassSet(class));

			Ok(().into()) //把错误装箱
		}

		#[pallet::weight(0)]
		pub fn set_student_info(
			origin: OriginFor<T>,
			student_number: u32,
			student_name: u128,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			//先判断
			if StudentInfo::<T>::contains_key(student_number) {
				return Err(Error::<T>::StudentInfoSetDuplicate.into());
			}

			StudentInfo::<T>::insert(&student_number, &student_name);

			//发出事件通知
			Self::deposit_event(Event::StudentInfoSet(student_number, student_name));

			Ok(().into())
		}
		#[pallet::weight(0)]
		pub fn set_dorm_info(
			origin: OriginFor<T>,
			dorm_number: u32,
			bed_number: u32,
			student_number: u32,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			//先判断
			if DormInfo::<T>::contains_key(dorm_number, bed_number) {
				return Err(Error::<T>::DormInfoSetDuplicate.into());
			}

			DormInfo::<T>::insert(&dorm_number, &bed_number, &student_number);

			//发出事件通知
			Self::deposit_event(Event::DormInfoSet(dorm_number, bed_number, student_number));

			Ok(().into())
		}
	}

	//步骤五：处理错误
	//和event类似，但error是当调度函数发生错误时发出的事件
	#[pallet::error]
	pub enum Error<T> {
		ClassSetDuplicate,
		StudentInfoSetDuplicate,
		DormInfoSetDuplicate,
	}
	//错误定义和Event类似
	//步骤六：使用钩子
	//例如在某两个步骤之间打印日志
	//fn offchain_worker(_n,BlockNumber) {...}
}
```

### **7.1.3 编译并运行**

```
cargo build --release
```

```
./target/release/node-template --tmp --dev
```

然后在前端使用sudo权限操作

## 7.2 Transaction

transaction需要导入才能使用	

```
#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*; //导出，外部可以调用
#[frame_support::pallet]
pub mod pallet {

	//1.引入外部依赖，可以引入其它的依赖
	use frame_support::pallet_prelude::*;
	use frame_support::transactional;
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
	}

	//4.存储，定义变量存放地方
	#[pallet::storage]
	//定义链上存储，有点同开辟存储空间
	//步骤二：定义存储，有四种：Storage Value、 Storage Map、Storage Double Map、Storage N Map
	//pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>; //前面两项是默认，后面两项是k,v
	#[pallet::getter(fn my_class)] //getter函数，与字段同名，只返回字段中的值，可以把字段变为私有，然后通过api访问
	pub type Class<T: Config> = StorageValue<_, u32>; //存储名：Class，存储类型：StorageValue，只存一个值（任何类型），默认要实现约束Config

	#[pallet::storage]
	#[pallet::getter(fn set_flag)]
	pub type HasSetFlag<T: Config> = StorageValue<_, Option<bool>>;

	// #[pallet::storage]
	// #[pallet::getter(fn student_info)] //ValueQuery:默认返回
	// pub type StudentInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>; //第二种存储类型，Map，k:StudentNumber,v:StudentName

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

	//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] //生成发出事件的函数
											  //步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		ClassSet(u32), //班级
		               //StudentInfoSet(u32, u128),  //学生信息
		               //DormInfoSet(u32, u32, u32), //寝室信息
	}

	//6.钩子，如一些固定的动作
	//#[pallet::hooks]
	//impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

	//7.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
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

		#[transactional] //自动回滚
		//每一个存储变量对应一个调度函数
		#[pallet::weight(0)] //所有调度函数都需要，操作成本，权重可以动态变化根据条件
				 //函数名称与存储名称在语义上保持统一，函数是对存储的操作，函数的结果使用Result枚举处理
		pub fn set_class_info(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			//1.判断条件: 签名、是否是root账户
			ensure_root(origin)?; //只有root账户才能操作

			// //1.先判断
			// if Class::<T>::exists() {
			// 	return Err(Error::<T>::ClassSetDuplicate.into());
			// }

			//2.操作
			//操作先于判断之前执行，但是由于用了transactional，所以判断失败后，状态会回滚
			Class::<T>::put(class); //StorageValue 使用put方法存储值，其他方法可以去官方文档查看

			if HasSetFlag::<T>::exists() {
				return Err(Error::<T>::FlagExisted.into());
			}

			HasSetFlag::<T>::put(Some(true));

			// if class <= 100u32 {
			// 	return Err(Error::<T>::NumberTooSmallThan100.into());
			// }

			// let _c = Self::my_class(); //调用getter函数

			//3.发出事件通知
			Self::deposit_event(Event::ClassSet(class));

			Ok(().into()) //把错误装箱
		}

		// #[pallet::weight(0)]
		// pub fn set_student_info(
		// 	origin: OriginFor<T>,
		// 	student_number: u32,
		// 	student_name: u128,
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
	}

	//步骤五：处理错误
	//和event类似，但error是当调度函数发生错误时发出的事件
	#[pallet::error]
	pub enum Error<T> {
		// ClassSetDuplicate,
		// NumberTooSmallThan100,
		// StudentInfoSetDuplicate,
		// DormInfoSetDuplicate,
		FlagExisted,
	}
	//错误定义和Event类似
	//步骤六：使用钩子
	//例如在某两个步骤之间打印日志
	//fn offchain_worker(_n,BlockNumber) {...}
}
```

# 8 钩子函数

## 8.1 区块打包过程

## 8.2 区块执行过程

**初始化区块**：执行所有pallet（construct_runtime!中包含的pallet，顺序如定义的一样）的on_initialize函数，不过会最先执行System模块的（frame- system）

**执行区块：**根据交易列表顺序执行

**确认区块：**调用所有pallet（同第一步）的on_idle和on_finalize函数，不过这次最后执行的System模块（frame- system）的hooks函数

## 8.3 hooks

hooks是一个trait,包含了很多函数

```

pub trait Hooks<BlockNumber> {
    fn on_finalize(_n: BlockNumber) { ... }
    fn on_idle(_n: BlockNumber, _remaining_weight: Weight) 
        -> Weight { ... }
    fn on_initialize(_n: BlockNumber) -> Weight { ... }
    fn on_runtime_upgrade() -> Weight { ... }
    fn pre_upgrade() -> Result<(), &'static str> { ... }
    fn post_upgrade() -> Result<(), &'static str> { ... }
    fn offchain_worker(_n: BlockNumber) { ... }
    fn integrity_test() { ... }
}
```

```
每个钩子函数在对应的时间自动调用执行，开发者可以根据需要在这些钩子函数中添加业务逻辑。

on_finalize: 在区块finalize的时候调用。
on_idle：区块finalize的时候调用，不过比on_finalize先调用。
on_initialize：区块初始化的时候调用。
on_runtime_upgrade：执行模块升级的时候调用。
pre_upgrade：升级之前的检查。
post_upgrade：升级之后的处理。
offchain_worker：在一个pallet上实现此函数后可以在此函数中长时间的执行需要链下执行的功能。该函数会在每次区块导入的时候调用。后续我们讲ocw使用的时候就需要和这个函数打交道。
integrity_test：运行集成测试。
```

**pallet完整代码**

```
#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*; //导出，外部可以调用
#[frame_support::pallet]
pub mod pallet {

	//1.引入外部依赖，可以引入其它的依赖
	use frame_support::pallet_prelude::*;
	use frame_support::transactional;
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
	}

	//4.存储，定义变量存放地方
	#[pallet::storage]
	//定义链上存储，有点同开辟存储空间
	//步骤二：定义存储，有四种：Storage Value、 Storage Map、Storage Double Map、Storage N Map
	//pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128>;
	// //前面两项是默认，后面两项是k,v
	#[pallet::getter(fn my_class)] //getter函数，与字段同名，只返回字段中的值，可以把字段变为私有，然后通过api访问
	pub type Class<T: Config> = StorageValue<_, u32>; //存储名：Class，存储类型：StorageValue，只存一个值（任何类型），默认要实现约束Config

	#[pallet::storage]
	#[pallet::getter(fn set_flag)]
	pub type HasSetFlag<T: Config> = StorageValue<_, Option<bool>>;

	// #[pallet::storage]
	// #[pallet::getter(fn student_info)] //ValueQuery:默认返回
	// pub type StudentInfo<T: Config> = StorageMap<_, Blake2_128Concat, u32, u128, ValueQuery>;
	// //第二种存储类型，Map，k:StudentNumber,v:StudentName

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

	//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] //生成发出事件的函数
											  //步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		ClassSet(u32), //班级
		//StudentInfoSet(u32, u128),  //学生信息
		//DormInfoSet(u32, u32, u32), //寝室信息
		SetParam(u32),
	}

	//7.错误
	//步骤五：处理错误
	//和event类似，但error是当调度函数发生错误时发出的事件
	#[pallet::error]
	pub enum Error<T> {
		// ClassSetDuplicate,
		NumberTooSmallThan100,
		// StudentInfoSetDuplicate,
		// DormInfoSetDuplicate,
		FlagExisted,
	}
	//错误定义和Event类似
	//步骤六：使用钩子
	//例如在某两个步骤之间打印日志
	//fn offchain_worker(_n,BlockNumber) {...}

	//6.钩子，如一些固定的动作
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		//实现两个函数
		fn on_finalize(n: BlockNumberFor<T>) {
			log::info!(target: "use-hooks","------- on_finalize,block number is {:?}",n);
		}
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			log::info!(target: "use-hooks","+++++++ on_initialize,block number is {:?}",n);
			0
		}
	}

	//7.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
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

		#[transactional] //自动回滚
		//每一个存储变量对应一个调度函数
		#[pallet::weight(0)] //所有调度函数都需要，操作成本，权重可以动态变化根据条件
				 //函数名称与存储名称在语义上保持统一，函数是对存储的操作，函数的结果使用Result枚举处理
		pub fn set_class_info(origin: OriginFor<T>, class: u32) -> DispatchResultWithPostInfo {
			//1.判断条件: 签名、是否是root账户
			ensure_root(origin)?; //只有root账户才能操作

			// //1.先判断
			// if Class::<T>::exists() {
			// 	return Err(Error::<T>::ClassSetDuplicate.into());
			// }

			//2.操作
			//操作先于判断之前执行，但是由于用了transactional，所以判断失败后，状态会回滚
			Class::<T>::put(class); //StorageValue 使用put方法存储值，其他方法可以去官方文档查看

			if HasSetFlag::<T>::exists() {
				return Err(Error::<T>::FlagExisted.into());
			}

			HasSetFlag::<T>::put(Some(true));

			// if class <= 100u32 {
			// 	return Err(Error::<T>::NumberTooSmallThan100.into());
			// }

			// let _c = Self::my_class(); //调用getter函数

			//3.发出事件通知
			Self::deposit_event(Event::ClassSet(class));

			Ok(().into()) //把错误装箱
		}

		// #[transactional]
		#[pallet::weight(0)]
		pub fn set_param_bigger_than_100(origin: OriginFor<T>, param: u32) -> DispatchResult {
			ensure_root(origin)?;

			if param <= 100 {
				return Err(Error::<T>::NumberTooSmallThan100.into());
			}

			Class::<T>::put(param);
			//3、发出事件
			Self::deposit_event(Event::SetParam(param));
			//添加log
			log::info!(target: "use-hooks", "set param bigger than 100");

			Ok(().into())
		}

		// #[pallet::weight(0)]
		// pub fn set_student_info(
		// 	origin: OriginFor<T>,
		// 	student_number: u32,
		// 	student_name: u128,
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
	}
}
```

# 9 Pallet Config

按照惯例改pallet名字和添加依赖

格式化

```
rustfmt +nightly 文件名
```

把参数改为泛型

在runtime的实现中指定

# 10 如何使用其它pallet

## 10.1 使用方法

三种使用方式：

- 在pallet的config中定义类型，然后在runtime中使用时指定这个类型为frame中个现成的pallet
- 在pallet的config中定义类型，然后在runtime中使用指定这个类型为某个自定义的pallet
- 封装和扩展现有pallet

如：

```
type Currency = Balances；
```

现在我们定义两个pallet，其中一个提供定义的存储，另一个通过trait调用

## 10.2 使用案例

**在一个pallet中使用另一个自定义的pallet**

先更新要使用到的两个pallet信息

```
[package]
name = "pallet-provide-pallet"
version = "0.0.1"
description = "example for provide"
```

```
[package]
name = "pallet-use-pallet"
version = "0.0.1"
description = "example for use storage"
```

**1.在traits.rs文件中定义trait**

```
//定一个一个trait，里面封装了在一个存储类型上执行操作的方法

pub trait StorageInterface {
	type Value;
	fn get_param() -> Self::Value;
	fn set_param(v: Self::Value);
}
```

**2. 定义provider-pallet**

```
#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*; //导出，外部可以调用
pub use traits::StorageInterface;

pub mod traits; //引入

#[frame_support::pallet]
pub mod pallet {

	//1.引入外部依赖，可以引入其它的依赖
	use super::*;
	use frame_support::dispatch::fmt::Debug;
	use frame_support::dispatch::Codec;
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::traits::AtLeast32BitUnsigned;
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
	#[pallet::storage]
	#[pallet::getter(fn my_value)] //ValueQuery:默认返回，T::Value 代表当前pallet的Value
	pub type MyValue<T: Config> = StorageValue<_, T::Value, ValueQuery>;

	//5.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] //生成发出事件的函数
	pub enum Event<T: Config> {
		FunctionCallSuccess,
	}
	
	//6.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
	#[pallet::call]
	impl<T: Config> Pallet<T> {
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
```

**3.定义Use- pallet**

```
#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use pallet::*; //导出，外部可以调用
#[frame_support::pallet]
pub mod pallet {

	//1.引入外部依赖，可以引入其它的依赖
	use super::*;
	use frame_support::dispatch::fmt::Debug;
	use frame_support::dispatch::Codec;
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::traits::AtLeast32BitUnsigned;
	//use frame_support::transactional;
	use frame_system::pallet_prelude::*;

	//1.引入pallet提供的trait‼️
	use pallet_provide_pallet::traits::StorageInterface;

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

		type Value: Member
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

		//定义关联类型，要求其实现该trait‼️
		type MyStorage: StorageInterface<Value = Self::Value>;
	}

	//4.链上事件的通知
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] //生成发出事件的函数
											  //步骤三：操作执行成功后通知用户
	pub enum Event<T: Config> {
		StoreEventSuccess,
	}

	//5.调度函数,类似于合约函数，pallet整个流程可以类比为一个智能合约，而合约的调用最终要在链上执行
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		//调用trait中的方法
		#[pallet::weight(0)]
		pub fn storage_value(
			origin: OriginFor<T>,
			value: T::Value,
		) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			//T代表当前pallet，
			T::MyStorage::set_param(value);
			//StudentInfo::<T>::insert(&student_number, &student_name);

			//使用trait StorageInterface中的函数
			let v = T::MyStorage::get_param();
			log::info!(target: "other-pallet", 
				"Value get from storage is: {:?}", v);
			Self::deposit_event(Event::StoreEventSuccess);

			Ok(().into())
		}
	}
}
```

**4.将provider添加至use-pallet依赖**

```
pallet-provide-pallet = {version = "0.0.1",default-features = false,path = "../provide-pallet"}
```

**5.将两个pallet更新至runtime中**

```
# Local Dependencies
--
pallet-use-pallet = { version = "0.0.1", default-features = false,path = "../pallets/use-pallet"}
pallet-provide-pallet = { version = "0.0.1", default-features = false,path = "../pallets/provide-pallet"}
```

```
[features]
default = ["std"]
std = [
--
"pallet-provide-pallet/std",
	"pallet-use-pallet/std",
]
```

```
//实现pallet
impl pallet_use_pallet::Config for Runtime {
	type Event = Event;
	type Value = u32;
	type MyStorage = StorageProvider;
}

//实现pallet
impl pallet_provide_pallet::Config for Runtime {
	type Event = Event;
	type Value = u32;
}
```

```
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		--
		StorageProvider:pallet_provide_pallet,
		StorageUser:pallet_use_pallet,
	}
);
```

6.编译并运行template，即可使用storageUser pallet调用 storageProvider pallet定义的存储存储数据

## 10.3 封装和扩展现有的pallet

调度体现在两个方面，一个是使用存储，另一个是使用调度函数

比如，在一个pallet中封装另一个pallet中的调度函数

## 10.3.1 编写extend pallet

**1.改名**

```
[package]
name = "pallet-extend-pallet"
version = "0.0.1"
description = "example for using another pallet's call function in this pallet"
```

**2.添加被调度的pallet作为依赖**

```
pallet-contracts = { default-features = false, version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.18" }
```

# 11.pallet 开发遇到的问题

怎么知道一个pallet能干哪些事情？通过polkadot.js 吗，它是暴露了一些函数？内置的pallet之间有套用么？去看一看

11.1 实现转账功能
