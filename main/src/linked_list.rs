pub mod prelude {
    pub use super::{Cons, Empty, List, cons};
}

pub enum List<T> {
    Cons(T, Box<List<T>>),
    Empty,
}

impl<T> List<T> {
    pub fn len(&self) -> usize {
        match self {
            Cons(_, list) => 1 + list.len(),
            Empty => 0,
        }
    }
    pub fn append(self, elem: T) -> Self {
        match self {
            Cons(x, list) => cons(x, list.append(elem)),
            Empty => cons(elem, Empty)
        }
    }
    pub fn map<R>(&self, f: impl Fn(&T) -> R) -> List<R> {
        match self {
            Cons(elem, others) => cons(f(elem), others.map(f)),
            Empty => Empty
        }
    }
    pub fn take(self, num: usize) -> List<T> {
        match (num, self) {
            (0, Cons(elem, others)) => cons(elem, Empty),
            (_, Cons(_, others)) => others.take(num - 1),
            (_, Empty) => Empty
        }
    }
    pub fn at(&self, index: usize) -> &T {
        match (index, self) {
            (0, Cons(x, _)) => x,
            (i, Cons(_, others)) => others.at(i - 1),
            (_, Empty) => panic!("Index out of range")
        }
    }
}

pub use List::{Cons, Empty};

pub fn cons<T>(elem: T, list: List<T>) -> List<T> {
    Cons(elem, Box::new(list))
}

pub mod test {
    use crate::go::Then;
    use crate::recipe::{Discard, Log};
    use super::{Cons, Empty, List, cons};

    struct A {
        string: String
    }

    impl A {
        fn new() -> Self {
            Self {string: "bum".to_string() }
        }
    }

    pub fn test1() {
        let a = cons(1, cons(2, Empty));
        let a = a.append(3);
        let b= a.map(|i| i * 2);
        println!("Len: {}", a.len());
        b.map(|i| println!("{i}"));
        println!("At 2: {}", b.at(2));
    }
    pub fn test2() {
        let a = cons(A::new(), cons(A::new(), Empty));
        let b = a.map(|e| e.string.clone() + "dee").take(1);
        b.map(|e| e.log().discard());
    }
}
