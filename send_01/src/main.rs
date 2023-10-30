// ## 1 没实现 Send 

// use std::rc::Rc;

// #[derive(Debug)]
// struct MyBox(*mut u8);

// unsafe impl Send for MyBox {
    
// }


// fn main() {
//     println!("Hello, world!");

//     let mb = MyBox(Box::into_raw(Box::new(42)));
//     let x = Rc::new(42);

    
//     std::thread::spawn(move || {
//         /*
//             因为 Rc 没有实现 Send trait
//             所以下边跨线程使用 x 时候就会报错
//          */
//         // println!("{:?}", x); // error: `Rc<i32>` cannot be sent between threads safely

//         // 而 MyBox 实现了 Send，所以这里不会报错
//         println!("{:?}", mb);
//     });

// }


// ## 2 没实现 Sync 

// use std::cell::RefCell;
// use std::env::consts::ARCH;
// use std::sync::Arc;

// fn main() {
//     let x = Arc::new(RefCell::new(42));
//     std::thread::spawn(move || {
//         let mut x = x.borrow_mut();
//         // 因为 RefCell 没有实现 Sync 所以下边报错
//         *x += 1; // error `RefCell<i32>` cannot be shared between threads safely
//     });
// }

// ## 3 实现 Clone 

// #[derive(Debug, Clone)]
// struct Person {
//     name: String,
//     age: u32,
// }

// impl Person {
//     fn new(name: String, age: u32) -> Person {
//         Person { name, age }
//     }
// }

// fn main() {
//     let p1 = Person::new("Alice".to_owned(), 22);
//     let p2 = p1.clone();

//     println!("p1: {:?}", p1);
//     println!("p2: {:?}", p2);
// }


// ## 4 实现 Default

// #[derive(Default)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let point = Point::default();
//     println!("Point: ({}, {})", point.x, point.y);
// }


// ## 5 实现 PartialEq


// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p1 = Point { x: 1, y: 2};
//     let p2 = Point { x: 1, y: 2};
//     let p3 = Point { x: 3, y: 4};

//     // 使用 == 比较它们是否相等
//     println!("Point1 == Point2: {}", p1 == p2);
//     println!("Point1 == Point3: {}", p1 == p3);
// }


// ## 6 实现 PartialOrd 

use std::collections::BTreeMap;

// 实现了这些 trait
// Ord 需要实现 PartialOrd
// #[derive(Debug, PartialEq, Eq, Clone)]
// struct Person {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let mut ages = BTreeMap::new();

//     let person1 = Person {
//         name: "Alice".to_owned(),
//         age: 25,
//     };

//     let person2 = Person {
//         name: "Bob".to_owned(),
//         age: 23,
//     };

//     let person3 = Person {
//         name: "Cook".to_owned(),
//         age: 31,
//     };

//     // 去掉实现 PartialOrd，这里报错
//     ages.insert(person1.clone(), "Alice's age");
//     ages.insert(person2.clone(), "Bob's age");
//     ages.insert(person3.clone(), "Cook's age");
 
//     for (person, desc) in &ages {
//         println!("{}: {} - {:?}", person.name, person.age, desc);
//     }
// }


// ## 6 实现 Hash

// use std::collections::HashSet;
// use std::hash::{Hash, Hasher};

// #[derive(Debug, PartialEq, Eq, Clone)]
// struct Person {
//     name: String,
//     age: u32,
// }

// impl Hash for Person {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//         self.age.hash(state);
//     }
// }

// fn main() {
//     let mut persons = HashSet::new();

//     let person1 = Person {
//         name: "Alice".to_owned(),
//         age: 30,
//     };

//     let person2 = Person {
//         name: "Bob".to_owned(),
//         age: 20,
//     };

//     let person3 = Person {
//         name: "Charlie".to_owned(),
//         age: 40,
//     };

//     persons.insert(person1.clone());
//     persons.insert(person2.clone());
//     persons.insert(person3.clone());

//     println!("Person Set {:?}", persons);

// }


// ## 7 copy 的意外

// #[derive(Debug, Copy, Clone)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p1 = Point { x: 10, y: 20 };
//     let p2 = p1; // Note：这里发送的是 Copy 而不是 Move

//     println!("p1: {:?}", p1);
//     println!("p2: {:?}", p2);
// }


// ## 8 包装类型

// use std::ops::Deref;

// struct MyVec(Vec<i32>);

// // 为 MyVec 实现 Deref，目标类型是 Vec<i32>
// impl Deref for MyVec {
//     type Target = Vec<i32>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     let my_vec = MyVec(vec![1, 2, 3, 4, 5]);
    
//     /*
//         由于 MyVec 实现了 Deref 
//         所以可以直接在 MyVec 类型的值上直接调用 Vec<i32> 的方法 len()
//         也可以通过索引获取第一个元素
//      */
//     println!("Length: {}", my_vec.len());
//     println!("First element: {}", my_vec[0]); 
// }

// # 9 包装类型实现 into from

// use std::ops::Deref;

// #[derive(Debug)]
// struct Wrapper(String);

// impl Deref for Wrapper {
//     type Target = String;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl AsRef<str> for Wrapper {
//     fn as_ref(&self) -> &str {
//         &self.0
//     }
// }

// // 为 Wrapper 实现 From，可以将 String 转为 Wrapper
// impl From<String> for Wrapper {
//     fn from(s: String) -> Self {
//         Wrapper(s)
//     }
// }

// impl From<Wrapper> for String {
//     fn from(wrapper: Wrapper) -> Self {
//         wrapper.0
//     }
// }

// fn main() {
//     let wrapper = Wrapper::from("Hello".to_string());

//     // 使用 . 运算符调用内部字符串类型的方法
//     println!("Length: {}", wrapper.len());

//     // 使用 as_ref 方法将 Wrapper 转换为 &str 类型
//     let inner_ref: &str = wrapper.as_ref();
//     println!("Inner: {}", inner_ref);

//     // 将 Wrapper 转换为内部类型 String
//     let inner_string: String = wrapper.into();
//     println!("Inner String: {}", inner_string);

//     // let w2: Wrapper = "World".to_string().into();
//     // println!("w2 Wrapper: {:?}", w2);

// }


// 10 borrow

use std::borrow::Borrow;

fn print_length<S>(string: S) where S: Borrow<str>, {
    println!("Length: {}", string.borrow().len());
}

fn main() {
    let str1: &str = "Hello";
    let string1: String = String::from("World");

    print_length(str1);
    print_length(string1);
}