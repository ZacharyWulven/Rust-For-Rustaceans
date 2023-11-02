
// 1 契约
// use std::borrow::Cow;

// // 契约更宽松
// fn frobnicate3<T: AsRef<str>>(s: T) -> T {
//     s
// }

// fn main() {
//     let string = String::from("example");
//     let borrowed: &str = "hello";
//     let cow: Cow<str> = Cow::Borrowed("world");

//     let result1: &str = frobnicate3::<&str>(string.as_ref());
//     let result2: &str = frobnicate3::<&str>(borrowed);
//     let result3: Cow<str> = frobnicate3(cow);

//     println!("Result1: {:?}", result1);
//     println!("Result2: {:?}", result2);
//     println!("Result3: {:?}", result3);
// }




// 2 使用泛型
// 有一个函数，它接收 AsRef<str> traiit 的参数
// fn print_as_str<T>(s: T) where T: AsRef<str> {
//     println!("{}", s.as_ref());
// }

/*
    这个函数是泛型的额，它对 T 进行了泛型化
    这意味着它会对你使用它的每一种实现了 AsRef<str> 的类型进行单态化
    例如 如果你用一个 String 和一个 &str 来调用它
    你就会在你的二进制文件中有两份函数的拷贝
*/


/*
    解决方法：改为接收 &dyn AsRef<str>
    这个函数就不再是泛型的了，它接受一个 trait 对象
    要求 s 是任何实现了 AsRef<str> 的类型
    这意味着它会在运行时使用动态分发来调用 as_ref 方法，
    并且你只会在你的二进制文件中有一份函数的拷贝
*/
// fn print_as_str2(s: &dyn AsRef<str>) {
//     println!("{}", s.as_ref());
// }



// fn main() {
//     let s = String::from("hello");
//     let r = "world";

//     // print_as_str(s); // 调用 print_as_str::<String>
//     // print_as_str(r); // 调用 print_as_str::<&str>

//     print_as_str2(&s); // 传递一个类型为 &dyn AsRef<str> 的 trait 对象
//     print_as_str2(&r); // 传递一个类型为 &dyn AsRef<str> 的 trait 对象
// }


// 3 静态分发、动态分发

/*
    静态分发：
    假设我们有一个名为 process 的泛型函数，它接受一个类型参数 T 并对其执行了某些操作
    这种函数就使用静态分发，这意味着在编译时将为每个具体类型 T 生成相应的实现
*/
// fn process<T>(value: T) {
//     // 处理 value 代码
//     println!("处理 T");
// }

/*
    动态分发：运行在运行时选择实现
    它们可以通过传递 Trait 对象作为参数，
    使用 dyn 关键字来实现，以下是一个例子
*/
// trait Processable {
//     fn process(&self);
// }

// #[derive(Debug)]
// struct TypeA;
// impl Processable for TypeA {
//     fn process(&self) {
//         println!("处理 TypeA");
//     }
// }

// struct TypeB;
// impl Processable for TypeB {
//     fn process(&self) {
//         println!("处理 TypeB");
//     }
// }

/*
    接收的类型为 trait 对象
    要求对象实现了 Processable
    如果调用者想要使用动态分发并在运行时选择实现
    它们可以调用 process_trait_object 函数，并传递 Trait 对象作为参数
    调用者可以根据需求选择要提供的具体实现 (面向协议编程)
*/
// fn process_trait_object(value: &dyn Processable) {
//     value.process();
// }


// fn main() {
//     let a = TypeA;
//     let b = TypeB;

//     // 以动态分发 方式调用
//     process_trait_object(&a);
//     process_trait_object(&b);

//     // 以静态分发 方式调用，传入不同类型产生不同的类型的实现
//     process(&a);
//     process(&b);
//     process(&a as &dyn Processable);
//     process(&b as &dyn Processable);

//     println!("TypeA {:?}", a);
// }


// 4 建议：应该从`具体类型`开始编写接口，然后逐渐将它们转为泛型 的例外

// fn foo(v: &Vec<usize>) {
//     // Code...
// }

// // 改为使用 Trait 限定 AsRef<[usize]> 即 impl AsRef<[usize]>
// fn foo2(v: impl AsRef<[usize]>) {
//     // Code...

// }

// fn main() {
    
//     let iter = vec![1, 2, 3].into_iter();

//     /*
//         这时调用 foo 没有问题，因为编译器可以推断出 iter.collect() 应该收集为一个 Vec<usize> 类型，
//         因为我们将其传递给了接受 &Vec<usize> 的 foo 函数
//      */
//     // foo(&iter.collect());

//     /*
//         然而改为 foo2 后, 编译器只知道 foo2 函数接受一个实现了 AsRef<[usize]> 特质的类型，
//         然而有多个类型满足这个条件，例如 Vec<usize> 和 &[usize]
//         因此编译器无法确定应该将 iter.collect() 的结果解释为那个具体类型
//         这就导致编译器无法推断类型，并且调用者代码将编译失败

//         而为了解决这个问题：
//         就需要指定一个确定的类型, foo2(&iter.collect::<Vec<usize>>())
//         或 
//         let list: Vec<usize> = iter.collect();
//         foo2(&list)
//      */
//     // let list: Vec<usize> = iter.collect();
//     // foo2(&list);
//     foo2(&iter.collect::<Vec<usize>>());

// }


// 5 对象安全的例子
/*
    假设我们有一个 Animal 特征，它有两个方法：name 和 speak
    name 方法返回一个 &str，表示动物名称
    speak 方法打印动物发出的声音
    我们可以为 Dog 和 Cat 类型实现这些特征

*/


// trait Animal {
//     fn name(&self) -> &str;
//     fn speak(&self);
// }

// struct Dog {
//     name: String
// }

// impl Animal for Dog {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("woof!");
//     }
// }

// struct Cat {
//     name: String
// }

// impl Animal for Cat {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("Meow!");
//     }
// }

// /*
//     这个 Animal 特征是 object-safe 的，因为它没有返回 Self 类型或使用泛型参数
//     所以我们可以用它来创建一个 trait 对象

//     这样我们就可以用一个统一的类型 Vec<&dyn Animal> 来存储不同类型的动物
//     并且通过 trait 对象来调用它们的方法
// */

// fn main() {
//     let dog = Dog { name: "Fido".to_string() };

//     let cat = Dog { name: "Whiskers".to_string() };

//     // 

//     let animals: Vec<&dyn Animal> = vec![&dog, &cat];

    
//     for animal in animals {
//         println!("This is {}", animal.name());
//         animal.speak();
//     }
// }


// 6 非对象安全的例子

/*
    如果 Animal 有返回 Self 类型，那它就不是对象安全的了
    那这个 Animal trait 就不是对象安全的，因为 clone 方法违反了规则（返回类型不能是 Self）
    现在就不能用它来创建 trait object 了，因为编译器无法知道 Self 具体指代哪个类型
*/
// trait Animal {
//     fn name(&self) -> &str;
//     fn speak(&self);
//     fn clone(&self) -> Self; 
// }

// struct Dog {
//     name: String,
// }

// impl Animal for Dog {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("wang wang!");
//     }

//     fn clone(&self) -> Self where Self: Sized, {
//         todo!()
//     }
// }

// struct Cat {
//     name: String,
// }

// impl Animal for Cat {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("miao miao!");
//     }

//     fn clone(&self) -> Self where Self: Sized, {
//         todo!()
//     }
// }

// fn main() {
//     let dog = Dog { name: "Fido".to_string() };

//     let cat = Dog { name: "Whiskers".to_string() };

//     let animals: Vec<&dyn Animal> = vec![&dog, &cat];

//     for animal in animals {
//         println!("This is {}", animal.name());
//     }
// }

// 6.2 改为保持对象安全

// trait Animal {
//     fn name(&self) -> &str;
//     fn speak(&self);
//     fn clone(&self) -> Self where Self: Sized; 
// }

// struct Dog {
//     name: String,
// }

// impl Animal for Dog {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("wang wang!");
//     }

//     fn clone(&self) -> Self where Self: Sized, {
//         todo!()
//     }
// }


// #[derive(Debug)]
// struct Cat {
//     name: String,
// }

// impl Animal for Cat {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("miao miao!");
//     }

//     fn clone(&self) -> Self where Self: Sized, {
//         Cat {
//             name: self.name.clone(),
//         }
//     }
// }

// fn main() {
//     let dog = Dog { name: "Fido".to_string() };

//     let cat = Cat { name: "Whiskers".to_string() };


//     /*
//         这样我们就可以继续用 Animal 来创建 trait object 了
//      */
//     let animals: Vec<&dyn Animal> = vec![&dog, &cat];

//     for animal in animals {
//         println!("This is {}", animal.name());
//         animal.speak();
//         /*
//             Note：但是我们不能用 trait object 来调用 clone 方法
//             因为只能在具体的类型上调用 clone 方法
//          */
//         //animal.clone();  // error
//     }

//     // 因为只能在具体的类型上调用 clone 方法
//     let cat2: Cat = cat.clone();
//     println!("Cloned cat is {:?}", cat2);
// }
    

// 7 泛型参数放在到 trait 上

// use std::collections::HashSet;
// use std::hash::Hash;

// // 将泛型参数放在 Trait 上
// trait Container<T> {
//     fn contains(&self, item: &T) -> bool;
// }

// // 我们可以为不同容器类型实现 Container Trait，每个实现都具有自己特定的元素类型
// // 例，可以为 Vec<T> 和 HashSet<T> 实现 Container Trait
// impl<T> Container<T> for Vec<T> 
//     where T: PartialEq, 
// {
//         fn contains(&self, item: &T) -> bool {
//             self.iter().any(|x| x == item)
//         }
// }

// impl<T> Container<T> for HashSet<T> 
// where T: Hash + Eq, 
// {   
//     fn contains(&self, item: &T) -> bool {
//         self.contains(item)
//     }
// }

// fn main() {
//     let vec_container: Box<dyn Container<i32>> = Box::new(vec![1, 2, 3]);

//     let set_container: Box<dyn Container<i32>> = Box::new(
//         vec![4, 5, 6].into_iter().collect::<HashSet<_>>()
//     );

//     // 调用 contains 方法

//     println!("Vec contains 2: {}", vec_container.contains(&2));
//     println!("HashSet contains 6: {}", set_container.contains(&6));
// }


// 8 泛型参数是否可以使用动态分发，来保证 `trait` 的对象安全

// use std::fmt::Debug;

// /*
//     有一个 trait Foo，它有一个泛型方法 bar，接收一个泛型参数 T：
//     trait Foo {
//         fn bar<T>(&self, x: T);
//     }
//     这个 Foo 是对象安全的么？答案是：取决于 T 的类型
//     1 如果 T是一个具体类型，比如 i32 或 String，那么它就 `不是` object-safe 的，
//     因为它需要再运行时知道 T 的具体类型才能调用 bar 方法

//     2 如果 T 是一个 trait object，比如 &dyn Debug 或 &dyn Display，
//     那么这个 trait 就是 object-safe 的，因为它可以用动态分发的方式来调用 T 的方法，
//     所以定义如下：
// */

// trait Foo {
//     fn bar(&self, x: &dyn Debug);
// }

// // 定义 A 让它实现 Foo
// struct A {
//     name: String,
// }

// impl Foo for A {
//     fn bar(&self, x: &dyn Debug) {
//         println!("A {} says {:?}", self.name, x);
//     }
// }

// // 定义 B 让它实现 Foo
// struct B {
//     id: i32,
// }

// impl Foo for B {
//     fn bar(&self, x: &dyn Debug) {
//         println!("B {} says {:?}", self.id, x);
//     }
// }

// fn  main() {
//     // 这时就可以用 Foo 来创建 trait 对象了

//     let a = A { name: "Bob".to_string() };

//     let b = B { id: 42 };

//     // 创建一个 Vec，它存储了 Foo 的 trait object
//     let foos: Vec<&dyn Foo> = vec![&a, &b];

//     // 遍历 Vec，并用 trait object 调用 bar 方法
//     for foo in foos {
//         // & 让 &str => &dyn Debug
//         foo.bar(&"Hello"); // "Hello" 实现了 Debug 特征
//     }
    
// }


// 9 通常在返回类型中使用 `Cow` 来表示有时会分配内存的函数

// use std::borrow::Cow;
// /*
//     有一个函数 process_data，它接收字符串参数
//     有时我们需要修改输入的字符串，并拥有对修改后的字符串的所有权
//     然而，大多数情况下，我们值对输入字符串进行读取操作，而不需要修改它
// */
// fn process_data(data: Cow<str>) {
//     if data.contains("invalid") {
//         // 如果输入字符串包含 "invalid"，我们需要修改它
//         // into_owned 获取其所有权，返回持有的数据
//         let owned_data: String = data.into_owned();
//         // 进行一些修改操作
//         println!("Processed data: {}", owned_data);
//     } else {
//         // 如果输入字符串不包含 "invalid"，我们只需要读取它
//         println!("Data: {}", data);
//     }
// }

// /*
//     本例中，我们使用 Cow<str> 类型作为参数类型
//     当调用时，我们可以传递一个普通的字符串引用（&str）
//     或一个拥有所有权的字符串（String）作为参数
// */

// fn main() {
//     let input1 = "This is valid data.";
//     process_data(Cow::Borrowed(input1));       // 传入引用

//     let input2 = "This is invalid data.";
//     process_data(Cow::Owned(input2.to_owned())); // 传入持有的数据
// }


// 10 析构函数

// use std::os::fd::AsRawFd;

// struct File {
//     name: String,
//     fd: i32,
// }

// impl File {
//     fn open(name: &str) -> Result<File, std::io::Error> {
//         // 使用 std::fs::OpenOptions 打开文件，具有读写权限
//         let file = std::fs::OpenOptions::new()
//             .read(true)
//             .write(true)
//             .open(name)?;

//         // 使用 std::os::unix::io::AsRawFd 获取文件描述符
//         let fd = file.as_raw_fd();
        
//         // 返回一个 File 实例，包含 name 和 fd 字段
//         Ok(File { 
//             name: name.to_string(),
//              fd, 
//         })
//     }

//     // 一个显示的析构函数，关闭文件并返回错误
//     fn close(self) -> Result<(), std::io::Error> {
//         // use std::os::unix::io::FromRawFd 将 fd 转回 std::fs::File
//         let file: std::fs::File = unsafe {
//             std::os::unix::io::FromRawFd::from_raw_fd(
//                 self.fd
//             )
//         };
//         // use std::fs::File::sync_all 将任何挂起的写入刷新到磁盘
//         file.sync_all()?;
//         // 使用 std::fs::File::set_len 将文件截断为 0 字节
//         file.set_len(0)?;
//         // 再次 use std::fs::File::sync_all 刷新截断
//         file.sync_all()?;

//         // 丢弃 file 实例，它会自动关闭
//         drop(file);

//         // 返回 Ok(())
//         Ok(())
//     }
// }

// fn main() {
//     // 创建一个名为 "test.txt" 的文件，包含一些内容
//     std::fs::write("test.txt", "Hello, world!").unwrap();

//     // 打开文件并获取一个 File 实例
//     let file = File::open("test.txt").unwrap();

//     println!("File name: {}, fd: {}", file.name, file.fd);

//     // 关闭文件并处理任何错误
//     // 收到调用析构函数 close
//     match file.close() {
//         Ok(_) => println!("File closed successfully"),
//         Err(e) => println!("Error closing file: {}", e),
//     }

//     // check 关闭后的文件大小
//     let metadata = std::fs::metadata("test.txt").unwrap();
//     println!("File size: {} bytes", metadata.len());
// }


// 11 Drop 不拥有 self

// use std::os::fd::AsRawFd;

// struct File {
//     name: String,
//     fd: i32,
// }

// impl File {
//     fn open(name: &str) -> Result<File, std::io::Error> {
//         // 使用 std::fs::OpenOptions 打开文件，具有读写权限
//         let file = std::fs::OpenOptions::new()
//             .read(true)
//             .write(true)
//             .open(name)?;

//         // 使用 std::os::unix::io::AsRawFd 获取文件描述符
//         let fd = file.as_raw_fd();
        
//         // 返回一个 File 实例，包含 name 和 fd 字段
//         Ok(File { 
//             name: name.to_string(),
//              fd, 
//         })
//     }

//     fn close(self) -> Result<(), std::io::Error> {
//         // 移出 name 字段并打印
//         let name = self.name; // 不能从 `self.name` 中移出值，因为它位于 `&mut` 引用后面
//         println!("Closing file {}", name);

//         // use std::os::unix::io::FromRawFd 将 fd 转回 std::fs::File
//         let file: std::fs::File = unsafe {
//             std::os::unix::io::FromRawFd::from_raw_fd(
//                 self.fd
//             )
//         };
//         // use std::fs::File::sync_all 将任何挂起的写入刷新到磁盘
//         file.sync_all()?;
//         // 使用 std::fs::File::set_len 将文件截断为 0 字节
//         file.set_len(0)?;
//         // 再次 use std::fs::File::sync_all 刷新截断
//         file.sync_all()?;

//         // 丢弃 file 实例，它会自动关闭
//         drop(file);

//         // 返回 Ok(())
//         Ok(())

//     }
// }

// // 为 File 实现 Drop trait，用于在值离开作用域时运行一些代码
// impl Drop for File {
//     // drop 方法，接受一个可变引用到 self 作为参数
//     fn drop(&mut self) {
//         /*
//             在 drop 中调用 close 方法并忽略它的结果

//             这里调用 close 报错，不能从 `*self` 中移出值，因为它位于 `&mut` 引用后面，
//             因为这里要获取其所有权
//             那这里怎么解决？没有完美的解决方案
//             方案一：


//          */
//         let _ = self.close(); 
//         // 打印一条消息，表明文件被丢弃了
//         println!("Dropping file {}", self.name);
//     }
// }


// 12 11 的解决方案一

// use std::os::fd::AsRawFd;

// // 一个表示文件句柄的类型
// struct File {
//     inner: Option<InnerFile>
// }

// // 一个内部类型，持有文件名和文件描述符，即原来的 File
// struct InnerFile {
//     name: String,
//     fd: i32,
// }

// impl File {
//     fn open(name: &str) -> Result<File, std::io::Error> {
//         // 使用 std::fs::OpenOptions 打开文件，具有读写权限
//         let file = std::fs::OpenOptions::new()
//             .read(true)
//             .write(true)
//             .open(name)?;

//         // 使用 std::os::unix::io::AsRawFd 获取文件描述符
//         let fd = file.as_raw_fd();
        
//         // 返回一个 File 实例，包含 name 和 fd 字段
//         Ok(File { 
//             inner: Some(InnerFile {
//                 name: name.to_string(),
//                 fd,
//             })
//         })
//     }
//     // 一个显示的析构函数，关闭文件并返回任何错误
//     fn close(mut self) -> Result<(), std::io::Error> {
//         // use Option::take 取出 inner 字段的值，并检查是否是 Some(InnerFile)
//         if let Some(inner) = self.inner.take() {
//         // 移出 name 和 fd 字段并打印
//         // 因为 inner 没有实现 drop trait，所以这里可以获取其所有权
//         let name = inner.name; 
//         let fd = inner.fd;
//         println!("Closing file {} with fd {}", name, fd);
//         // use std::os::unix::io::FromRawFd 将 fd 转回 std::fs::File
//         let file: std::fs::File = unsafe {
//             std::os::unix::io::FromRawFd::from_raw_fd(
//                 fd
//             )
//         };
//         // use std::fs::File::sync_all 将任何挂起的写入刷新到磁盘
//         file.sync_all()?;
//         // 使用 std::fs::File::set_len 将文件截断为 0 字节
//         file.set_len(0)?;
//         // 再次 use std::fs::File::sync_all 刷新截断
//         file.sync_all()?;

//         // 丢弃 file 实例，它会自动关闭
//         drop(file);

//         // 返回 Ok(())
//         Ok(())

//         } else {
//             // 如果 inner 字段是 None，说明文件已经被关闭或丢弃了，返回一个 error
//             Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "File already closed or dropped",
//             ))
//         }
//     }
// }

// // 为 File 实现 Drop trait，用于在值离开作用域时运行一些代码
// impl Drop for File {
//     // drop 方法，接受一个可变引用到 self 作为参数
//     fn drop(&mut self) {
//         if let Some(inner) = self.inner.take() {
//             let name = inner.name;
//             let fd = inner.fd;
//             println!("Closing file {} with fd {}", name, fd);
//             // use std::os::unix::io::FromRawFd 将 fd 转回 std::fs::File
//             let file: std::fs::File = unsafe {
//                 std::os::unix::io::FromRawFd::from_raw_fd(
//                     fd
//                 )
//             };
//             drop(file);
//         } else {
//             // 如果 inner 字段是 None，说明文件已经被关闭或丢弃了
//         }
//     }
// }

// // 用于测试 File 类型的主函数
// fn main() {
//     // 创建一个名为 "test.txt" 的文件，包含一些内容
//     std::fs::write("test.txt", "Hello, world!").unwrap();

//     // 打开文件并获取一个 File 实例
//     let file = File::open("test.txt").unwrap();

//     println!(
//         "File name: {}, fd: {}", 
//         file.inner.as_ref().unwrap().name, 
//         file.inner.as_ref().unwrap().fd
//     );

//     // 关闭文件并处理任何错误
//     // 收到调用析构函数 close
//     match file.close() {
//         Ok(_) => println!("File closed successfully"),
//         Err(e) => println!("Error closing file: {}", e),
//     }

//     // check 关闭后的文件大小
//     let metadata = std::fs::metadata("test.txt").unwrap();
//     println!("File size: {} bytes", metadata.len());
// }

// 12 11 的解决方案二

// use std::os::fd::AsRawFd;

// struct File {
//     name: Option<String>,
//     fd: Option<i32>,
// }

// impl File {
//     fn open(name: &str) -> Result<File, std::io::Error> {
//         // 使用 std::fs::OpenOptions 打开文件，具有读写权限
//         let file = std::fs::OpenOptions::new()
//             .read(true)
//             .write(true)
//             .open(name)?;

//         // 使用 std::os::unix::io::AsRawFd 获取文件描述符
//         let fd = file.as_raw_fd();
        
//         // 返回一个 File 实例，包含 name 和 fd 字段
//         Ok(File { 
//             name: Some(name.to_string()),
//             fd: Some(fd),
//         })
//     }

//     // 一个显示的析构函数，关闭文件并返回任何错误
//     fn close(mut self) -> Result<(), std::io::Error> {
//         // use std::mem::take 取出 name 字段的值，并检查是否是 Some(name)
//         if let Some(name) = std::mem::take(&mut self.name) {
//         // use std::mem::take 取出 fd 字段的值，并检查是否是 Some(fd)
//             if let Some(fd) = std::mem::take(&mut self.fd) {

//                 println!("Closing file {} with fd {}", name, fd);
//                 // use std::os::unix::io::FromRawFd 将 fd 转回 std::fs::File
//                 let file: std::fs::File = unsafe {
//                     std::os::unix::io::FromRawFd::from_raw_fd(
//                         fd
//                     )
//                 };
//                 // use std::fs::File::sync_all 将任何挂起的写入刷新到磁盘
//                 file.sync_all()?;
//                 // 使用 std::fs::File::set_len 将文件截断为 0 字节
//                 file.set_len(0)?;
//                 // 再次 use std::fs::File::sync_all 刷新截断
//                 file.sync_all()?;

//                 // 丢弃 file 实例，它会自动关闭
//                 drop(file);

//                 // 返回 Ok(())
//                 Ok(())
//             } else {
//                 // 如果 inner 字段是 None，说明文件已经被关闭或丢弃了，返回一个 error
//                 Err(std::io::Error::new(
//                     std::io::ErrorKind::Other,
//                     "File already closed or dropped",
//                 ))
//             }
//         } else {
//             // 如果 inner 字段是 None，说明文件已经被关闭或丢弃了，返回一个 error
//             Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "File already closed or dropped",
//             ))
//         }
//     }
// }

// // 为 File 实现 Drop trait，用于在值离开作用域时运行一些代码
// impl Drop for File {
//     // drop 方法，接受一个可变引用到 self 作为参数
//     fn drop(&mut self) {
//         if let Some(name) = std::mem::take(&mut self.name) {
//             if let Some(fd) = std::mem::take(&mut self.fd) {
//                 // 打印
//                 println!("Closing file {} with fd {}", name, fd);
//                 // use std::os::unix::io::FromRawFd 将 fd 转回 std::fs::File
//                 let file: std::fs::File = unsafe {
//                     std::os::unix::io::FromRawFd::from_raw_fd(
//                         fd
//                     )
//                 };
//                 drop(file);
//             }
//         } else {
//             // 如果 fd 字段是 None，说明文件已经被关闭或丢弃了
//         }
//     }
// }

// // 用于测试 File 类型的主函数
// fn main() {
//     // 创建一个名为 "test.txt" 的文件，包含一些内容
//     std::fs::write("test.txt", "Hello, world!").unwrap();

//     // 打开文件并获取一个 File 实例
//     let file = File::open("test.txt").unwrap();

//     println!(
//         "File name: {}, fd: {}", 
//         file.name.as_ref().unwrap(), 
//         file.fd.as_ref().unwrap()
//     );

//     // 关闭文件并处理任何错误
//     // 收到调用析构函数 close
//     match file.close() {
//         Ok(_) => println!("File closed successfully"),
//         Err(e) => println!("Error closing file: {}", e),
//     }

//     // check 关闭后的文件大小
//     let metadata = std::fs::metadata("test.txt").unwrap();
//     println!("File size: {} bytes", metadata.len());
// }


// 13 11 的解决方案三

use std::{os::fd::AsRawFd, mem::ManuallyDrop};

struct File {
    // 包装在 ManuallyDrop 中
    name: ManuallyDrop<String>,
    fd: ManuallyDrop<i32>,
}

impl File {
    fn open(name: &str) -> Result<File, std::io::Error> {
        // 使用 std::fs::OpenOptions 打开文件，具有读写权限
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(name)?;

        // 使用 std::os::unix::io::AsRawFd 获取文件描述符
        let fd = file.as_raw_fd();
        
        // 返回一个 File 实例，包含 name 和 fd 字段
        Ok(File { 
            name: ManuallyDrop::new(name.to_string()),
            fd: ManuallyDrop::new(fd),
        })
    }

    // 一个显示的析构函数，关闭文件并返回任何错误
    fn close(mut self) -> Result<(), std::io::Error> {
        // use std::mem::replace 将 name 字段替换为空字符串，并获取原来的值 
        // 或用  String::new() 代替 "".to_string()
        let name = std::mem::replace(&mut self.name, ManuallyDrop::new("".to_string()));

        // use std::mem::replace 将 fd 字段替换为无需的值，并获取原来的值
        let fd = std::mem::replace(&mut self.fd, ManuallyDrop::new(-1));

        println!("close Closing file {:?} with fd {:?}", name, fd);

        // use std::os::unix::io::FromRawFd 将 fd 转回 std::fs::File
        let file: std::fs::File = unsafe {
            std::os::unix::io::FromRawFd::from_raw_fd(
                *fd
            )
        };
        // use std::fs::File::sync_all 将任何挂起的写入刷新到磁盘
        file.sync_all()?;
        // 使用 std::fs::File::set_len 将文件截断为 0 字节
        file.set_len(0)?;
        // 再次 use std::fs::File::sync_all 刷新截断
        file.sync_all()?;

        // 丢弃 file 实例，它会自动关闭
        drop(file);

        // 返回 Ok(())
        Ok(())


    }
}

// 为 File 实现 Drop trait，用于在值离开作用域时运行一些代码
impl Drop for File {
    // drop 方法，接受一个可变引用到 self 作为参数
    fn drop(&mut self) {
        // 使用 ManuallyDrop::take 取出 name 字段的值，并检查是否是空字符串
        let name = unsafe { ManuallyDrop::take(&mut self.name) };

        // 使用 ManuallyDrop::take 取出 fd 字段的值，并检查是否是无效的值
        let fd = unsafe { ManuallyDrop::take(&mut self.fd) };

        println!("drop Closing file {} with fd {}", name, fd);

        // 如果 fd 不是无效的值，说明文件还没有被关闭或丢弃，需要执行一些操作
        if fd != -1 {
            let file: std::fs::File = unsafe {
                std::os::unix::io::FromRawFd::from_raw_fd(
                    fd
                )
            };
            // 丢弃 file 实例，它会自动关闭
            drop(file);

        }
    }
}

// 用于测试 File 类型的主函数
fn main() {
    // 创建一个名为 "test.txt" 的文件，包含一些内容
    std::fs::write("test.txt", "Hello, world!").unwrap();

    // 打开文件并获取一个 File 实例
    let file = File::open("test.txt").unwrap();

    println!(
        "File name: {}, fd: {}", 
        *file.name, 
        *file.fd
    );

    // 关闭文件并处理任何错误
    // 收到调用析构函数 close
    match file.close() {
        Ok(_) => println!("File closed successfully"),
        Err(e) => println!("Error closing file: {}", e),
    }

    // check 关闭后的文件大小
    let metadata = std::fs::metadata("test.txt").unwrap();
    println!("File size: {} bytes", metadata.len());
}