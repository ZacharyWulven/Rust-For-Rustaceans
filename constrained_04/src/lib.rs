// 2 用户的代码不仅仅通过名称依赖于你的类型

// pub struct Unit {
//     field: bool,
// }


// 3 #[non_exhaustive] 解决 例2 的问题

// #[non_exhaustive]
// pub struct Config {
//     pub window_width: u16,
//     pub window_height: u16,
// }


// fn some_function() {
//     let config = Config {
//         window_width: 640,
//         window_height: 480,
//     };

//     // non_exhaustive struct 可以使用这种详尽的方式创建
//     if let Config { 
//         window_width, 
//         window_height
//     } = config {
//         // ....
//     }

// }


// 4 为现有类型实现任何 `Trait` 都要小心

// pub struct Unit;

// pub trait Foo1 {
//     fn foo(&self);
// }

// // // case1: add impl Foo1 for Unit in this crate
// // impl Foo1 for Unit {
// //     fn foo(&self) {
// //         println!("foo1 is called");
// //     }
// // }

// // case2: add a new public trait
// pub trait Bar1 {
//     fn foo(&self);
// }

// impl Bar1 for Unit {
//     fn foo(&self) {
//         println!("bar1");
//     }
// }

// 5 Sealed Trait
// use std::fmt::{Debug, Display};

// mod sealed {
//     use std::fmt::{Debug, Display};

//     pub trait Sealed {}
//     impl<T> Sealed for T where T: Debug + Display {
        
//     }
// }

// /*
//     这个是想封闭的 trait，它有一个 super trait 
//     super trait 在一个私有模块下，所以其他 crate 无法访问它
//     实现 CanUseCannotImplement 的还必须实现 sealed::Sealed
//     下边我们实现 sealed::Sealed
// */
// pub trait CanUseCannotImplement: sealed::Sealed {
//     // ...
// }

// // 这里实现了 Sealed，T 是 Debug + Display
// impl<T> CanUseCannotImplement for T where T: Debug + Display {
    
// }


// 6 重新导出
// // 你的 crate 叫 bestiter
// pub fn iter<T>() -> itercrate::Empty<T> { 
//     //...
// }

// // 依赖外部 crate，叫 itercrate（v1.0），提供了 Empty<T> 类型

// // 用户的 crate 中
// // 用户使用了 你的 crate 的 Empty<T> 类型，
// // 但 Empty<T> 类型是 itercrate（v1.0）中定义的
// struct EmptyIterator { it: itercrate::Empty<()>}

// EmptyIterator { it: bestiter:: iter() }

// // -----------------------------------------------
// // 你的 crate，叫 bestiter
// pub fn iter<T>() -> itercrate::Empty<T> { 
//     //...
// }

// // 依赖外部 crate，叫 itercrate，提供了 Empty<T> 类型
// // 你把依赖 itercrate 版本改为 v2.0, 其他地方没有改
// // 但这时用户的 crate 代码就会报错，问什么？
// // 因为，编译器认为：itercrate1.0::Empty 和 itercrate2.0::Empty 是不同类型
// // 导致破坏性变更


// // 用户的 crate 中
// struct EmptyIterator { it: itercrate::Empty<()>}


// 7 在 crate 中 包含测试以便检查你的类型

fn is_normal<T>() where T: Sized + Send + Sync + Unpin {

}

#[test]
fn normal_types() {
    // MyType 应该实现了 Sized + Send + Sync + Unpin
    // 如果编译通过了，则说明 MyType 实现了上述 Traits
    // 如果编译失败了，则说明有些没有实现
    is_normal::<MyType>();
}

