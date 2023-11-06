// 1 pub(crate)

// pub mod outer_mod {
//     pub mod inner_mod {
//         // This function is visible within `outer_mod`
//         // 只对 mod outer_mod 可见
//         pub(in crate::outer_mod) fn outer_mod_visible_fn() {}

//         // This function is visible to the entire crate
//         // 整个 crate 都可见
//         pub(crate) fn crate_visible_fn() {}

//         // This function is visible within `outer_mod`
//         // super is outer_mod
//         pub(super) fn super_mod_visible_fn() {
//             // This function is visible since we're in the same `mod`
//             inner_mod_visible_fn();
//         }

//         // This function is visible only within `inner_mod`,
//         // which is the same as leaving it private.
//         // 即当前 inner_mod 可见，就类似于是私有的
//         pub(self) fn inner_mod_visible_fn() {}
//     }

//     pub fn foo() {
//         inner_mod::outer_mod_visible_fn();
//         inner_mod::crate_visible_fn();
//         inner_mod::super_mod_visible_fn();

//         // Error! inner_mod_visible_fn is private
//         // inner_mod::inner_mod_visible_fn();
//     }
// }


// fn bar() {
//     outer_mod::inner_mod::crate_visible_fn();

//     // Error! super_mod_visible_fn is private
//     outer_mod::inner_mod::super_mod_visible_fn();

//     // Error! outer_mod_visible_fn is private
//     outer_mod::inner_mod::outer_mod_visible_fn();

//     outer_mod::foo(); 
// }


// fn main() {
//     bar();
// }


// 2 用户的代码不仅仅通过名称依赖于你的类型

// fn is_true(u: constrained_04::Unit) -> bool {
//     matches!(u, constrained_04::Unit { field: true })
// }

// fn main() {
//     // 引用 lib.rs 里的 Unit
//     // 用户原来代码，Unit 中没有任何字段
//     // Unit 添加 field 字段后，这里就报错了
//     // 无论 field 是 pub 还是 private 的都会报错
//     let u = constrained_04::Unit;
// }


// 3 #[non_exhaustive] 解决 例2 的问题

// use constrained_04::Config;
// fn main() {
//     // 在 非 non_exhaustive 定义的 crate，这样创建就不行了
//     // Error! 
//     let config = Config {
//         window_width: 640,
//         window_height: 480,
//     };

//     // 在 非 non_exhaustive 定义的 crate，这样创建就不行了
//     if let Config { 
//         window_width, 
//         window_height,
//         ..       // 这里必须加上 `..` 表示忽略其他的字段才 work，但上边的构造方式就不被允许
//     } = config {
//         // ....
//     }

// }

// 4 为现有类型实现任何 `Trait` 都要小心

// case1
// use constrained_04::{Foo1, Unit};

// case2
// use constrained_04::*;

// // case1 & case2

// trait Foo2 {
//     fn foo(&self);
// }

// impl Foo2 for Unit {
//     fn foo(&self) {
//         println!("foo2 is called");
//     }
// }

// fn main() {
//     /*
//         case 1: Error! 由于 lib.rs 中实现了 Foo1 中有 foo 方法，
//         而上边又对 Foo2 进行了实现，
//         重名了所以这里就报错了
//      */
//     Unit.foo();

// }


// 5 Sealed Trait
use std::fmt::{Debug, Display};
use constrained_04::CanUseCannotImplement;

// Bar 实现了 Debug 和 Display
pub struct Bar {

}

impl Debug for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

// Error! 因为 lib.rs 已经实现了 CanUseCannotImplement，它就不能实现了
impl CanUseCannotImplement for Bar {
    
}

pub struct Foo {}

// Error! 这里 Foo 没有实现 Debug 和 Display 所以也报错
impl CanUseCannotImplement for Foo {
    
}

fn main() {

}

// 6 重新导出
