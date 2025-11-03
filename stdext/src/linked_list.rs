pub mod prelude {
    pub use super::{Cons, Empty, List, cons};
}

use replace_with::replace_with_or_abort;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, format};
use std::ops::{Index, IndexMut};

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
    pub fn is_empty(&self) -> bool {
        match self {
            Empty => true,
            _ => false,
        }
    }
    pub fn push(&mut self, elem: T) {
        match self {
            Cons(_, list) => list.push(elem),
            Empty => replace_with_or_abort(self, |this| cons(elem, Empty)),
        }
    }
    pub fn push_front(&mut self, elem: T) {
        replace_with_or_abort(self, |this| cons(elem, this));
    }
    pub fn back(&mut self) -> &mut Self {
        match self {
            Cons(_, list) if list.is_empty() => self,
            Cons(_, list) => list.back(),
            Empty => self,
        }
    }
    pub fn append(&mut self, other: Self) {
        replace_with_or_abort(self.back(), |back| match back {
            Cons(element, _) => cons(element, other),
            Empty => other,
        });
    }
    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq,
    {
        match self {
            Cons(elem, others) => elem == x || others.contains(x),
            Empty => false,
        }
    }
    pub fn starts_with(&self, needle: &Self) -> bool
    where
        T: PartialEq,
    {
        match self {
            Cons(x, xs) => match needle {
                Cons(y, ys) => x == y && xs.starts_with(ys),
                Empty => false,
            },
            Empty => needle.is_empty(),
        }
    }
    pub fn iter(&self) -> ListRefIter<T> {
        match self {
            Cons(head, tail) => ListRefIter::Cons(Some(head), tail),
            Empty => ListRefIter::Empty,
        }
    }
    pub fn iter_mut(&mut self) -> ListRefMutIter<'_, T> {
        match self {
            Cons(head, tail) => ListRefMutIter::Cons(Some(head), tail),
            Empty => ListRefMutIter::Empty,
        }
    }
}

impl<T: Debug> List<T> {
    pub fn join(&self, separator: impl AsRef<str>) -> String {
        let separator = separator.as_ref();
        match self {
            Cons(elem, tail) if tail.is_empty() => format!("{elem:?}"),
            Cons(elem, tail) => format!("{elem:?}{}{}", separator, tail.join(separator)),
            Empty => "".to_string(),
        }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Cons(elem, others) => ListIter::Cons(Some(elem), others),
            Empty => ListIter::Empty,
        }
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.join(", "))
    }
}

pub enum ListIter<T> {
    Cons(Option<T>, Box<List<T>>),
    Empty,
}

impl<T> Iterator for ListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ListIter::Cons(elem, others) => {
                let ret = elem.take().unwrap();
                replace_with_or_abort(self, |this| match this {
                    ListIter::Cons(_, others) => others.into_iter(),
                    ListIter::Empty => unreachable!(),
                });
                Some(ret)
            }
            ListIter::Empty => None,
        }
    }
}

pub enum ListRefIter<'a, T> {
    Cons(Option<&'a T>, &'a Box<List<T>>),
    Empty,
}

impl<'a, T> Iterator for ListRefIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ListRefIter::Cons(elem, others) => {
                let ret = elem.take().unwrap();
                replace_with_or_abort(self, |this| match this {
                    ListRefIter::Cons(_, others) => others.iter(),
                    ListRefIter::Empty => unreachable!(),
                });
                Some(ret)
            }
            ListRefIter::Empty => None,
        }
    }
}

pub enum ListRefMutIter<'a, T> {
    Cons(Option<&'a mut T>, &'a mut Box<List<T>>),
    Empty,
}

impl<'a, T> Iterator for ListRefMutIter<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ListRefMutIter::Cons(elem, others) => {
                let ret = elem.take().unwrap();
                replace_with_or_abort(self, |this| match this {
                    ListRefMutIter::Cons(_, others) => others.iter_mut(),
                    ListRefMutIter::Empty => unreachable!(),
                });
                Some(ret)
            }
            ListRefMutIter::Empty => None,
        }
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match (index, self) {
            (0, Cons(elem, _)) => elem,
            (n, Cons(_, others)) => others.index(n - 1),
            (_, Empty) => panic!("Index out of range"),
        }
    }
}

impl<T> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match (index, self) {
            (0, Cons(elem, _)) => elem,
            (n, Cons(_, others)) => others.index_mut(n - 1),
            (_, Empty) => panic!("Index out of range"),
        }
    }
}

impl<T> PartialEq for List<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            Cons(x, xs) => match other {
                Cons(y, ys) => x == y && xs == ys,
                Empty => false,
            },
            Empty => match other {
                Cons(_, _) => false,
                Empty => true,
            },
        }
    }
}

impl<T> PartialOrd for List<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Cons(x, xs) => match other {
                Cons(y, ys) => {
                    if x == y {
                        xs.partial_cmp(ys)
                    } else {
                        x.partial_cmp(y)
                    }
                }
                Empty => Some(Ordering::Greater),
            },
            Empty => match other {
                Cons(_, _) => Some(Ordering::Less),
                Empty => Some(Ordering::Equal),
            },
        }
    }
}

impl<T> Eq for List<T> where T: Eq {}
impl<T> Ord for List<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Cons(x, xs) => match other {
                Cons(y, ys) => {
                    if x == y {
                        xs.cmp(ys)
                    } else {
                        x.cmp(y)
                    }
                }
                Empty => Ordering::Greater,
            },
            Empty => match other {
                Cons(_, _) => Ordering::Less,
                Empty => Ordering::Equal,
            },
        }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<It: IntoIterator<Item = T>>(iter: It) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            Some(element) => cons(element, List::from_iter(iter)),
            None => Empty,
        }
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<It: IntoIterator<Item = T>>(&mut self, iter: It) {
        self.append(iter.into_iter().collect());
    }
}

pub use List::{Cons, Empty};

pub fn cons<T>(head: T, tail: List<T>) -> List<T> {
    Cons(head, Box::new(tail))
}

#[macro_export]
macro_rules! link {
    // Base case: no elements
    [] => {
        $crate::linked_list::List::Empty
    };
    // Recursive case: at least one element
    [$head:expr $(, $tail:expr)* $(,)?] => {
        $crate::linked_list::cons($head, link![$($tail),*])
    };
}

pub mod test {
    use super::{Empty, cons};

    struct A {
        string: String,
    }

    impl A {
        fn new() -> Self {
            Self {
                string: "bum".to_string(),
            }
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
        a.push(4);
        println!("a at 3: {}", a[3]);
        let b = link![1, 2, 3];
        b.into_iter().for_each(|i| println!("{i}"));
        let mut c = link!["a".to_string(), "b".to_string(), "c".to_string()];
        c.iter_mut().for_each(|mut s| *s += "1");
        c.iter().for_each(|s| println!("{s}"));
    }
    pub fn test3() {
        let mut a = link![1, 2, 3];
        a.push_front(0);
    }
}
