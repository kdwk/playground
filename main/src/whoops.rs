use std::{error::Error, fmt::Display};

/// NoneError: Expected Some(...), got None.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NoneError;

impl Error for NoneError {
    /// "NoneError: Expected Some(...), got None."
    fn description(&self) -> &str {
        "NoneError: Expected Some(...), got None."
    }
}

impl Display for NoneError {
    /// "NoneError: expected Some(...), got None."
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("NoneError: expected Some(...), got None.")
    }
}

pub type Whoops = Result<(), Box<dyn Error>>;

pub trait IntoWhoops {
    fn into_whoops(self) -> Whoops;
}

impl IntoWhoops for () {
    fn into_whoops(self) -> Whoops {
        Ok(())
    }
}

impl<T> IntoWhoops for Result<T, Box<dyn Error>> {
    fn into_whoops(self) -> Whoops {
        match self {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}

impl<T> IntoWhoops for Option<T> {
    fn into_whoops(self) -> Whoops {
        match self {
            Some(_) => Ok(()),
            None => Err(NoneError)?,
        }
    }
}

pub fn attempt<Closure, Return>(closure: Closure) -> Whoops
where
    Closure: FnOnce() -> Return,
    Return: IntoWhoops,
{
    closure().into_whoops()
}

pub trait Catch {
    fn catch<HandleErrorClosure: FnOnce(Box<dyn Error>)>(self, closure: HandleErrorClosure);
}

impl<T: IntoWhoops> Catch for T {
    fn catch<HandleErrorClosure: FnOnce(Box<dyn Error>)>(self, closure: HandleErrorClosure) {
        match self.into_whoops() {
            Ok(_) => {}
            Err(error) => closure(error),
        }
    }
}
