use std::{any::Any, borrow::Borrow, collections::HashMap, hash::Hash, error::Error, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use extend::ext;

pub mod prelude {
    pub use super::{any, Anything, AnythingExt, Mixture, MixtureExt};
}

#[macro_export]
macro_rules! mix {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut mixture = Mixture::new();
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
    fn get<T: 'a>(&self) -> &T;
    fn get_mut<T: 'a>(&mut self) -> &mut T;
    fn try_get<T: 'a>(&self) -> Option<&T>;
    fn try_get_mut<T: 'a>(&mut self) -> Option<&mut T>;
    fn set<T: 'a>(&mut self, value: T);
    fn case<T: 'a>(&mut self, match_arm: impl FnOnce(&mut T)) -> (&mut Anything<'a>, bool);
}

impl<'a> AnythingExt<'a> for Anything<'a> {
    fn get<T: 'a>(&self) -> &T {
        (*self).downcast_ref().unwrap()
    }
    fn get_mut<T: 'a>(&mut self) -> &mut T {
        (*self).downcast_mut().unwrap()
    }
    fn try_get<T: 'a>(&self) -> Option<&T> {
        (*self).downcast_ref()
    }
    fn try_get_mut<T: 'a>(&mut self) -> Option<&mut T> {
        (*self).downcast_mut()
    }
    fn set<T: 'a>(&mut self, value: T) {
        *self = any(value);
    }
    fn case<T: 'a>(&mut self, match_arm: impl FnOnce(&mut T)) -> (&mut Anything<'a>, bool) {
        let mut matched = false;
        if let Some(value) = self.try_get_mut::<T>() {
            match_arm(value);
            matched = true;
        }
        (self, matched)
    }
}

#[macro_export]
macro_rules! mixedmap {
    ($( $x:expr => $y:expr ),* $(,)?) => {
        {
            HashMap::from([
                $(($x, any($y)),)*
            ])
        }
    };
}

#[ext]
impl<'a, Key: Hash + Eq, Output> HashMap<Key, Anything<'a>> {
    fn get_any<Q: ?Sized>(&'a self, k: &Q) -> Option<&'a Output>
    where
        Key: Borrow<Q>,
        Q: Hash + Eq {
            Some(self.get(k)?.get())
        }
    fn get_any_mut<Q: ?Sized>(&'a mut self, k: &Q) -> Option<&'a mut Output>
    where
        Key: Borrow<Q>,
        Q: Hash + Eq {
            Some(self.get_mut(k)?.get_mut())
        }
}

impl<'a> AnythingExt<'a> for (&mut Anything<'a>, bool) {
    fn get<T: 'a>(&self) -> &T {
        self.0.get()
    }
    fn get_mut<T: 'a>(&mut self) -> &mut T {
        self.0.get_mut()
    }
    fn try_get<T: 'a>(&self) -> Option<&T> {
        self.0.try_get()
    }
    fn try_get_mut<T: 'a>(&mut self) -> Option<&mut T> {
        self.0.try_get_mut()
    }
    fn set<T: 'a>(&mut self, value: T) {
        self.0.set(value);
    }
    fn case<T: 'a>(&mut self, match_arm: impl FnOnce(&mut T)) -> (&mut Anything<'a>, bool) {
        if !self.1 {
            if let Some(value) = self.try_get_mut::<T>() {
                match_arm(value);
                self.1 = true;
            }
        }
        (self.0, self.1)
    }
}

pub trait Number: AddAssign + SubAssign + MulAssign + DivAssign + Sized + Add + Sub + Mul + Div {}

impl<T: AddAssign + SubAssign + MulAssign + DivAssign + Sized + Add + Sub + Mul + Div> Number for T {}

mod test {
    use std::ops::Add;
    use super::Number;

    fn add<Num: Number>(x: Num, y: Num) -> <Num as Add>::Output {
        x + y
    }
    fn test1() {
        println!("{}", add(4, 3));
    }
}