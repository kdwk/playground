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
        Anything, AnythingExt, AnythingMutExt, HashMapKeyAnythingaExt, Mixture, MixtureExt, any,
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

pub trait AnythingExt<'a> {
    #[inline]
    fn get<T: 'a>(&self) -> &T {
        self.try_get().unwrap()
    }
    fn try_get<T: 'a>(&self) -> Option<&T>;
    fn case<T: 'a>(&self, match_arm: impl FnOnce(&T)) -> Option<&Anything<'a>>;
    #[inline]
    fn default(&self, default_arm: impl FnOnce()) {
        default_arm();
    }
}

pub trait AnythingMutExt<'a>: AnythingExt<'a> {
    #[inline]
    fn get_mut<T: 'a>(&mut self) -> &mut T {
        self.try_get_mut().unwrap()
    }
    fn try_get_mut<T: 'a>(&mut self) -> Option<&mut T>;
    fn set<T: 'a>(&mut self, value: T);
    fn case_mut<T: 'a>(&mut self, match_arm: impl FnOnce(&mut T)) -> Option<&mut Anything<'a>>;
}

impl<'a> AnythingExt<'a> for Anything<'a> {
    #[inline]
    fn try_get<T: 'a>(&self) -> Option<&T> {
        self.downcast_ref()
    }
    #[inline]
    fn case<T: 'a>(&self, match_arm: impl FnOnce(&T)) -> Option<&Anything<'a>> {
        match self.try_get() {
            Some(value) => {
                match_arm(value);
                None
            }
            None => Some(self),
        }
    }
}

impl<'a> AnythingMutExt<'a> for Anything<'a> {
    #[inline]
    fn try_get_mut<T: 'a>(&mut self) -> Option<&mut T> {
        self.downcast_mut()
    }
    #[inline]
    fn set<T: 'a>(&mut self, value: T) {
        *self = any(value);
    }
    #[inline]
    fn case_mut<T: 'a>(&mut self, match_arm: impl FnOnce(&mut T)) -> Option<&mut Anything<'a>> {
        match self.try_get_mut() {
            Some(value) => {
                match_arm(value);
                None
            }
            None => Some(self),
        }
    }
}

impl<'a> AnythingExt<'a> for Option<&Anything<'a>> {
    #[inline]
    fn try_get<T: 'a>(&self) -> Option<&T> {
        self.and_then(|anything| anything.downcast_ref())
    }
    #[inline]
    fn case<T: 'a>(&self, match_arm: impl FnOnce(&T)) -> Option<&Anything<'a>> {
        match self.try_get() {
            Some(value) => {
                match_arm(value);
                None
            }
            None => *self,
        }
    }
}

impl<'a> AnythingExt<'a> for Option<&mut Anything<'a>> {
    #[inline]
    fn try_get<T: 'a>(&self) -> Option<&T> {
        self.as_ref().and_then(|anything| anything.downcast_ref())
    }
    #[inline]
    fn case<T: 'a>(&self, match_arm: impl FnOnce(&T)) -> Option<&Anything<'a>> {
        match self.try_get() {
            Some(value) => {
                match_arm(value);
                None
            }
            None => self.as_ref().and_then(|ref_mut_ref| Some(&**ref_mut_ref)),
        }
    }
}

impl<'a> AnythingMutExt<'a> for Option<&mut Anything<'a>> {
    #[inline]
    fn try_get_mut<T: 'a>(&mut self) -> Option<&mut T> {
        self.as_mut().and_then(|anything| anything.downcast_mut())
    }
    #[inline]
    fn set<T: 'a>(&mut self, value: T) {
        if let Some(this) = self {
            **this = any(value);
        }
    }
    #[inline]
    fn case_mut<T: 'a>(&mut self, match_arm: impl FnOnce(&mut T)) -> Option<&mut Anything<'a>> {
        match self.try_get_mut() {
            Some(value) => {
                match_arm(value);
                None
            }
            None => self
                .as_mut()
                .and_then(|mut_ref_mut_ref| Some(&mut **mut_ref_mut_ref)),
        }
    }
}

impl<'a> AnythingExt<'a> for Option<Anything<'a>> {
    #[inline]
    fn try_get<T: 'a>(&self) -> Option<&T> {
        self.as_ref().and_then(|anything| anything.downcast_ref())
    }
    #[inline]
    fn case<T: 'a>(&self, match_arm: impl FnOnce(&T)) -> Option<&Anything<'a>> {
        match self.try_get() {
            Some(value) => {
                match_arm(value);
                None
            }
            None => (*self).as_ref(),
        }
    }
}

impl<'a> AnythingMutExt<'a> for Option<Anything<'a>> {
    #[inline]
    fn try_get_mut<T: 'a>(&mut self) -> Option<&mut T> {
        self.as_mut().and_then(|anything| anything.downcast_mut())
    }
    #[inline]
    fn set<T: 'a>(&mut self, value: T) {
        if let Some(this) = self {
            *this = any(value);
        }
    }
    #[inline]
    fn case_mut<T: 'a>(&mut self, match_arm: impl FnOnce(&mut T)) -> Option<&mut Anything<'a>> {
        match self.try_get_mut() {
            Some(value) => {
                match_arm(value);
                None
            }
            None => (*self).as_mut(),
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
        self.get(&k).unwrap().get()
    }
    fn try_get_any<Output>(&'a self, k: &Q) -> Option<&'a Output>
    where
        Key: Borrow<Q>,
        Q: Hash + Eq,
    {
        Some(self.get(&k)?.get())
    }
    fn try_get_any_mut<Output>(&'a mut self, k: &Q) -> Option<&'a mut Output>
    where
        Key: Borrow<Q>,
        Q: Hash + Eq,
    {
        Some(self.get_mut(&k)?.get_mut())
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
