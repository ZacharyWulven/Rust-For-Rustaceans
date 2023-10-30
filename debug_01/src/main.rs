use std::fmt;

struct Pair<T> {
    a: T,
    b: T,
}

// 这里即手段实现 Debug trait
impl<T: fmt::Debug> fmt::Debug for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pair")
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}

fn main() {
    let pair = Pair { a: 5, b: 10};

    println!("Pair: {:?}", pair);
}
