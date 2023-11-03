
// 1 文档标记 panic

// use std::error::Error;

/// 除法运行，返回两个数的结果
/// 
/// # Panics
/// 
/// 如果除数为零，该函数会发生 panic。
/// 
/// # 示例
/// 
/// ````
/// let result = divide(10, 2);
/// assert_eq!(result, 5);
/// ````
// pub fn divide(dividend: i32, divisor: i32) -> i32 {
//     // Code...
//     todo!()
// }


// 2 文档 #[doc(hidden)]，用于生成文档时，不会显示该标记的方法、函数、结构体等的文档

// 一个简单的模块，包含一些用于内部使用的函数和结构体
// pub mod internal {
//     /// 一个拥有内部计算辅助函数 (因为标记了 #[doc(hidden)]，所以这个句不会出现在文档中)
//     #[doc(hidden)]
//     pub fn internal_helper() {
//         // 内部计算实现...
//     }

//     /// 一个仅用于内部使用的结构体
//     #[doc(hidden)]
//     pub struct InternalStruct {
//         // 字段和方法
//     }
// }

// // 一个公共接口函数，调用了内部辅助函数
// pub fn public_function() {
//     // 调用内部辅助函数
//     internal::internal_helper();
// }


// 3 #[doc(alias = "")]

///! 这是一个用于处理图像的库
///!
///! 这个库提供了一些常用的图像处理功能，例如
///! - 读取和保存不同格式的图像文件 [`Image::load`] [`Image::save`]
///! - 调整图像的大小、旋转和裁剪 [`Image::resize`] [`Image::rotate`] [`Image::crop`]
///! - 应用不同的滤镜和效果 [`Filter`] [`Effect`]
///! 
///! 如果您想了解更多关于图像处理的原理和算法，您可以参考以下资源：
///! - [数字图像处理](https://book.xxx.com/subject/5345798)，一本经典教科书，介绍图像处理的基本概念
///! - [Learn OpenCV](https://learnopencv.com)，一个网站，提供很多用 OpenCV 实现图像处理功能的教程和示例代码
///! - [Awsome Computer Vision](https://github.com/jbhuang0604/awesome-computer-vision)，一个仓库 

/// 一个表示图像的结构体
// #[derive(Debug, Clone)]
// pub struct Image {

// }

// impl Image {
//     /// 从指定路径加载一个图像文件
//     /// 
//     /// 支持的格式有：PNG、JPEG、GIF、BMP 等
//     /// 
//     /// # 参数
//     /// 
//     /// - `path`: 图像文件的路径
//     /// 
//     /// # 返回值
//     /// 
//     /// 如果成功，返回一个 [`Image`] 实例；如果失败，返回一个 [`Error`]
//     /// 
//     /// # 示例
//     /// 
//     /// ```no_run
//     /// use image::Image;
//     /// 
//     /// let img = Image::load("test.png")?;
//     /// ```
//     #[doc(alias = "读取")]
//     #[doc(alias = "打开")]
//     pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
//         todo!()
//     }
// }

// 4 #[doc(cfg(...))]

// 一个只在启用了 `foo` 特性时才可用的结构体
// #[cfg(feature = "foo")]
// #[doc(cfg(feature = "foo"))]
// pub struct Foo;

// impl Foo {
//     // 一个只在启用了 `foo` 特性时才可用的方法
//     #[cfg(feature = "foo")]
//     #[doc(cfg(feature = "foo"))]
//     pub fn bar(&self) {
//         // ...
//     }
// }


// 5 类型系统
/*
    这里参数是 3 个 bool，用户可能会把其含义给记混了
*/
// fn process_data(dryRun: bool, overwrite: bool, validate: bool) {
//     // Code...
// }

// enum DryRun {
//     Yes,
//     No,
// }

// enum Overwriite {
//     Yes,
//     No,
// }

// enum Validate {
//     Yes,
//     No,
// }
// /*
//     可以将 bool 定义为枚举，这样更有语义化
//     用户在调用时候就不容易出错
// */
// fn process_data2(dryRun: DryRun, overwrite: Overwriite, validate: Validate) {
//     // Code...

// }


// 6 使用 `零大小的类型` 来表示关于类型实例的特定事实

// struct Grounded;

// struct Launched;

// enum Color {
//     White,
//     Black,
// }

// struct Kilograms(u32);

// // 这里泛型不用 T，用 Stage 表示
// // 这里表示 只有 Stage 在 Grounded 时才能创建 🚀
// struct Rocket<Stage = Grounded> {
//     /*
//         PhantomData 在没编译完时候就相当于里边的 Stage
//         而编译完就没有了，就相当于一个单元类型
//         它的作用就是在不同的条件下限制这个火箭类型的行为
//      */
//     stage: std::marker::PhantomData<Stage>,
// }

// impl Default for Rocket<Grounded> {
//     fn default() -> Self {
//         Self { 
//             stage: Default::default(),
//         }
//     }
// }

// // 这里表示只有 Stage 在 Grounded 时才能发射 🚀

// impl Rocket<Grounded> {
//     pub fn launch(self) -> Rocket<Launched> {
//         Rocket { stage: Default::default() }
//     }
// }

// // 这里表示只有 🚀 发射后，才能调用加速、减速

// impl Rocket<Launched> {
//     pub fn accelerate(&mut self) {}
//     pub fn decelerate(&mut self) {}
// }

// // 这些方法在任何阶段都可以调用
// impl<Stage> Rocket<Stage> {

//     pub fn color(&self) -> Color {
//         Color::White
//     }

//     pub fn weight(&self) -> Kilograms {
//         Kilograms(0)
//     }

// }


// 7 `#[must_use]` 注解

use std::error::Error;


/*
    使用 #[must_use] 注解，表示必须使用 process_data 函数的返回值
    如果没有使用其返回值，编译器就会发生警告
    这有助于提醒用户在处理潜在的错误情况时要小心，并减少可能的错误
*/
#[must_use]
fn process_data(data: Data) -> Result<(), Error> {

    Ok(())
}





fn main() {
    //process_data2(DryRun::No, Overwriite::Yes, Validate::No);
}
