use std::{any::Any, error::Error};

#[macro_export]
macro_rules! mix {
    ( $( $x:expr ),* ) => {
        {
            let mut mixture = Mixture::new();
            $(
                mixture.add($x);
            )*
            mixture
        }
    };
}

pub type Anything = Box<dyn Any>;
pub type Mixture = Vec<Anything>;

pub fn any<T: 'static>(thing: T) -> Anything {
    Box::new(thing)
}

pub trait MixtureExt {
    fn add<T: 'static>(&mut self, value: T);
    fn extract<T: 'static>(&mut self, index: usize) -> T;
    fn pop<T: 'static>(&mut self) -> T;
}

impl MixtureExt for Mixture {
    fn add<T: 'static>(&mut self, value: T) {
        self.push(any(value))
    }
    fn extract<T: 'static>(&mut self, index: usize) -> T {
        let value_ref = self.remove(index);
        match value_ref.downcast::<T>() {
            Ok(value) => *value,
            Err(_) => panic!(),
        }
    }
    fn pop<T: 'static>(&mut self) -> T {
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

pub trait AnythingExt {
    fn get<T: 'static>(&mut self) -> &mut T;
    fn try_get<T: 'static>(&mut self) -> Option<&mut T>;
    fn set<T: 'static>(&mut self, value: T);
    fn try_set<T: 'static>(&mut self, value: T) -> Result<(), Box<dyn Error>>;
}

impl AnythingExt for Anything {
    fn get<T: 'static>(&mut self) -> &mut T {
        match (*self).downcast_mut::<T>() {
            Some(value) => value,
            None => panic!(),
        }
    }
    fn try_get<T: 'static>(&mut self) -> Option<&mut T> {
        (*self).downcast_mut::<T>()
    }
    fn set<T: 'static>(&mut self, value: T) {
        *self = any(value);
    }
    fn try_set<T: 'static>(&mut self, value: T) -> Result<(), Box<dyn Error>> {
        self.set(value);
        Ok(())
    }
}

/// A trait for matching a the type of a value against a number of types.
/// Match arm of first match is run.
/// Specify matching types by type annotation on match arm closure arguments.
pub trait MatchType {
    /// Match type of self against 1 type
    fn match_type1<A: 'static>(&mut self, arm1: impl FnOnce(&mut A));
    /// Match type of self against 2 types
    fn match_type2<A: 'static, B: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
    );
    /// Match type of self against 3 types
    fn match_type3<A: 'static, B: 'static, C: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
        arm3: impl FnOnce(&mut C),
    );
    /// Match type of self against 4 types
    fn match_type4<A: 'static, B: 'static, C: 'static, D: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
        arm3: impl FnOnce(&mut C),
        arm4: impl FnOnce(&mut D),
    );
    /// Match type of self against 5 types
    fn match_type5<A: 'static, B: 'static, C: 'static, D: 'static, E: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
        arm3: impl FnOnce(&mut C),
        arm4: impl FnOnce(&mut D),
        arm5: impl FnOnce(&mut E),
    );
    /// Match type of self against 6 types
    fn match_type6<A: 'static, B: 'static, C: 'static, D: 'static, E: 'static, F: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
        arm3: impl FnOnce(&mut C),
        arm4: impl FnOnce(&mut D),
        arm5: impl FnOnce(&mut E),
        arm6: impl FnOnce(&mut F),
    );
}

impl MatchType for Box<dyn Any> {
    fn match_type1<A: 'static>(&mut self, arm1: impl FnOnce(&mut A)) {
        if let Some(value) = self.try_get::<A>() {
            arm1(value);
        }
    }
    fn match_type2<A: 'static, B: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
    ) {
        if let Some(value) = self.try_get::<A>() {
            arm1(value);
        } else if let Some(value) = self.try_get::<B>() {
            arm2(value);
        }
    }
    fn match_type3<A: 'static, B: 'static, C: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
        arm3: impl FnOnce(&mut C),
    ) {
        if let Some(value) = self.try_get::<A>() {
            arm1(value);
        } else if let Some(value) = self.try_get::<B>() {
            arm2(value);
        } else if let Some(value) = self.try_get::<C>() {
            arm3(value);
        }
    }
    fn match_type4<A: 'static, B: 'static, C: 'static, D: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
        arm3: impl FnOnce(&mut C),
        arm4: impl FnOnce(&mut D),
    ) {
        if let Some(value) = self.try_get::<A>() {
            arm1(value);
        } else if let Some(value) = self.try_get::<B>() {
            arm2(value);
        } else if let Some(value) = self.try_get::<C>() {
            arm3(value);
        } else if let Some(value) = self.try_get::<D>() {
            arm4(value);
        }
    }
    fn match_type5<A: 'static, B: 'static, C: 'static, D: 'static, E: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
        arm3: impl FnOnce(&mut C),
        arm4: impl FnOnce(&mut D),
        arm5: impl FnOnce(&mut E),
    ) {
        if let Some(value) = self.try_get::<A>() {
            arm1(value);
        } else if let Some(value) = self.try_get::<B>() {
            arm2(value);
        } else if let Some(value) = self.try_get::<C>() {
            arm3(value);
        } else if let Some(value) = self.try_get::<D>() {
            arm4(value);
        } else if let Some(value) = self.try_get::<E>() {
            arm5(value);
        }
    }
    fn match_type6<A: 'static, B: 'static, C: 'static, D: 'static, E: 'static, F: 'static>(
        &mut self,
        arm1: impl FnOnce(&mut A),
        arm2: impl FnOnce(&mut B),
        arm3: impl FnOnce(&mut C),
        arm4: impl FnOnce(&mut D),
        arm5: impl FnOnce(&mut E),
        arm6: impl FnOnce(&mut F),
    ) {
        if let Some(value) = self.try_get::<A>() {
            arm1(value);
        } else if let Some(value) = self.try_get::<B>() {
            arm2(value);
        } else if let Some(value) = self.try_get::<C>() {
            arm3(value);
        } else if let Some(value) = self.try_get::<D>() {
            arm4(value);
        } else if let Some(value) = self.try_get::<E>() {
            arm5(value);
        } else if let Some(value) = self.try_get::<F>() {
            arm6(value);
        }
    }
}
