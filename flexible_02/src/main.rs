
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


fn foo(v: &Vec<usize>) {
    // Code...
}

// 改为使用 Trait 限定 AsRef<[usize]> 即 impl AsRef<[usize]>
fn foo2(v: impl AsRef<[usize]>) {
    // Code...

}

fn main() {
    
    let iter = vec![1, 2, 3].into_iter();

    /*
        这时调用 foo 没有问题，因为编译器可以推断出 iter.collect() 应该收集为一个 Vec<usize> 类型，
        因为我们将其传递给了接受 &Vec<usize> 的 foo 函数
     */
    // foo(&iter.collect());

    /*
        然而改为 foo2 后, 编译器只知道 foo2 函数接受一个实现了 AsRef<[usize]> 特质的类型，
        然而有多个类型满足这个条件，例如 Vec<usize> 和 &[usize]
        因此编译器无法确定应该将 iter.collect() 的结果解释为那个具体类型
        这就导致编译器无法推断类型，并且调用者代码将编译失败

        而为了解决这个问题：
        就需要指定一个确定的类型, foo2(&iter.collect::<Vec<usize>>())
        或 
        let list: Vec<usize> = iter.collect();
        foo2(&list)
     */
    // let list: Vec<usize> = iter.collect();
    // foo2(&list);
    foo2(&iter.collect::<Vec<usize>>());

}

