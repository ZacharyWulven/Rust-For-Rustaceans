use std::fmt::Debug;

/*
    Pair 使用派生的方式实现 trait，所以 T 要实现 Debug trait
*/
#[derive(Debug)]
struct Pair<T> {
    a: T,
    b: T,
}

struct Person {
    name: String,
}

fn main() {
    let pair = Pair { a: 5, b: 10 };
    // i32 已经实现了 Debug trait，所以下边打印没有问题
    println!("Pair: {:?}", pair);

    let pair = Pair {
        a: Person { name: "Dave".to_string() },
        b: Person { name: "Tom".to_string() },
    };
    // 由于 Person 没有实现 Debug trait，所以这里打印报错
    println!("Pair: {:?}", pair);

}
