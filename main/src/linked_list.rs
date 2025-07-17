pub mod prelude {
    pub use super::{Cons, Empty, List, cons};
}

use std::ops::{Index, IndexMut};
use replace_with::replace_with_or_abort;

pub enum List<T> {
    Cons(T, Box<List<T>>),
    Empty,
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Cons(elem, others) => ListIter::Cons(Some(elem), others),
            Empty => ListIter::Empty
        }
    }
}

pub enum ListIter<T> {
    Cons(Option<T>, Box<List<T>>),
    Empty
}

impl<T> Iterator for ListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ListIter::Cons(elem, others) => {
                let ret = elem.take().unwrap();
                replace_with_or_abort(self, |this| match this {
                    ListIter::Cons(_, others) => others.into_iter(),
                    ListIter::Empty => unreachable!()
                });
                Some(ret)
            }
            ListIter::Empty => None
        }
    }
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
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match (index, self) {
            (0, Cons(elem, _)) => elem,
            (n, Cons(_, others)) => others.index(n - 1),
            (_, Empty) => panic!("Index out of range")
        }
    }
}

impl<T> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match (index, self) {
            (0, Cons(elem, _)) => elem,
            (n, Cons(_, others)) => others.index_mut(n - 1),
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
        let a = cons(1, cons(2, cons(3, Empty)));
        a.into_iter().for_each(|i| println!("{i}"));
    }
    pub fn test2() {
        let mut a = cons(1, cons(2, cons(3, Empty)));
        println!("a at 2: {}", a[2]);
        a[2] = 4;
        println!("a at 2: {}", a[2]);
    }
}
