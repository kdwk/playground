use std::{
    any::Any,
    borrow::Borrow,
    collections::HashMap,
    hash::Hash,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use extend::ext;

pub mod prelude {
    pub use super::{
        Anything, HashMapKeyAnythingaExt, Mixture, MixtureExt, any, switch, switch_mut,
    };
}

#[macro_export]
macro_rules! mix {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut mixture = $crate::mixture::Mixture::new();
            $(
                mixture.add($x);
            )*
            mixture
        }
    };
}

pub type Anything<'a> = Box<dyn Any + 'a>;
pub type Mixture<'a> = Vec<Anything<'a>>;

pub fn any<'a, T: 'a>(thing: T) -> Anything<'a> {
    Box::new(thing)
}

pub trait MixtureExt {
    fn add<T: 'static>(&mut self, value: T);
    fn extract<T: 'static>(&mut self, index: usize) -> T;
    fn pop<T: 'static>(&mut self) -> T;
}

impl<'a> MixtureExt for Mixture<'a> {
    fn add<T: 'a>(&mut self, value: T) {
        self.push(any(value))
    }
    fn extract<T: 'a>(&mut self, index: usize) -> T {
        let value_ref = self.remove(index);
        match value_ref.downcast::<T>() {
            Ok(value) => *value,
            Err(_) => panic!(),
        }
    }
    fn pop<T: 'a>(&mut self) -> T {
        let boxed_value = match self.pop() {
            Some(boxed_value) => boxed_value,
            None => panic!(),
        };
        match boxed_value.downcast::<T>() {
            Ok(value) => *value,
            Err(_) => panic!(),
        }
    }
}

pub enum AnythingSwitcher<'a, Ret> {
    BeforeMatch(&'a Box<dyn Any>),
    AfterMatch(Ret),
}

#[inline]
pub fn switch<'a, Ret>(anything: &'a Anything) -> AnythingSwitcher<'a, Ret> {
    AnythingSwitcher::BeforeMatch(anything)
}

impl<'a, Ret> AnythingSwitcher<'a, Ret> {
    #[inline]
    pub fn case<T: 'static>(self, match_arm: impl FnOnce(&T) -> Ret) -> Self {
        match self {
            Self::BeforeMatch(any) => match any.is::<T>() {
                true => Self::AfterMatch(match_arm(any.downcast_ref().unwrap())),
                false => self,
            },
            Self::AfterMatch(_) => self,
        }
    }
    #[inline]
    pub fn default(self, default_arm: impl FnOnce() -> Ret) -> Ret {
        match self {
            Self::BeforeMatch(_) => default_arm(),
            Self::AfterMatch(ret) => ret,
        }
    }
    #[inline]
    pub fn ret(self) -> Option<Ret> {
        match self {
            Self::BeforeMatch(_) => None,
            Self::AfterMatch(ret) => Some(ret),
        }
    }
}

pub enum AnythingSwitcherMut<'a, Ret> {
    BeforeMatch(&'a mut Box<dyn Any>),
    AfterMatch(Ret),
}

#[inline]
pub fn switch_mut<'a, Ret>(anything: &'a mut Anything) -> AnythingSwitcherMut<'a, Ret> {
    AnythingSwitcherMut::BeforeMatch(anything)
}

impl<'a, Ret> AnythingSwitcherMut<'a, Ret> {
    #[inline]
    pub fn case<T: 'static>(self, match_arm: impl FnOnce(&mut T) -> Ret) -> Self {
        match self {
            Self::BeforeMatch(any) => match any.is::<T>() {
                true => Self::AfterMatch(match_arm(any.downcast_mut().unwrap())),
                false => Self::BeforeMatch(any),
            },
            Self::AfterMatch(_) => self,
        }
    }
    #[inline]
    pub fn default(self, default_arm: impl FnOnce() -> Ret) -> Ret {
        match self {
            Self::BeforeMatch(_) => default_arm(),
            Self::AfterMatch(ret) => ret,
        }
    }
    #[inline]
    pub fn ret(self) -> Option<Ret> {
        match self {
            Self::BeforeMatch(_) => None,
            Self::AfterMatch(ret) => Some(ret),
        }
    }
}

#[macro_export]
macro_rules! mixedmap {
    ($( $x:expr => $y:expr ),* $(,)?) => {
        {
            use std::collections::HashMap;
            HashMap::from([
                $(($x, any($y)),)*
            ])
        }
    };
}

#[ext(pub)]
impl<'a, Key: Hash + Eq, Q: ?Sized> HashMap<Key, Anything<'a>> {
    fn get_any<Output>(&'a self, k: &Q) -> &'a Output
    where
        Key: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.get(&k).unwrap().downcast_ref().unwrap()
    }
    fn try_get_any<Output>(&'a self, k: &Q) -> Option<&'a Output>
    where
        Key: Borrow<Q>,
        Q: Hash + Eq,
    {
        Some(self.get(&k)?.downcast_ref().unwrap())
    }
    fn try_get_any_mut<Output>(&'a mut self, k: &Q) -> Option<&'a mut Output>
    where
        Key: Borrow<Q>,
        Q: Hash + Eq,
    {
        Some(self.get_mut(&k)?.downcast_mut().unwrap())
    }
}

pub trait Number:
    AddAssign + SubAssign + MulAssign + DivAssign + Sized + Add + Sub + Mul + Div
{
}

impl<T: AddAssign + SubAssign + MulAssign + DivAssign + Sized + Add + Sub + Mul + Div> Number
    for T
{
}

mod test {
    use super::Number;
    use std::ops::Add;

    fn add<Num: Number>(x: Num, y: Num) -> <Num as Add>::Output {
        x + y
    }
    fn test1() {
        println!("{}", add(4, 3));
    }
}
