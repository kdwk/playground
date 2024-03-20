use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct NoneError;

impl Error for NoneError {
    fn description(&self) -> &str {
        "NoneError: Expected Some(...), got None."
    }
}

impl Display for NoneError {
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

impl IntoWhoops for Whoops {
    fn into_whoops(self) -> Whoops {
        self
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

pub fn attempt<Closure, Arg, Return>(closure: Closure) -> Closure
where
    Closure: FnOnce(Arg) -> Return,
    Return: IntoWhoops,
{
    closure
}

pub trait Catch<Arg> {
    fn catch<HandleErrorClosure, HandleErrorClosureReturn>(
        self,
        closure: HandleErrorClosure,
    ) -> impl FnOnce(Arg) -> Whoops
    where
        HandleErrorClosure: FnOnce(&Box<dyn Error>) -> HandleErrorClosureReturn,
        HandleErrorClosureReturn: IntoWhoops;
}

impl<Closure, Arg, Return> Catch<Arg> for Closure
where
    Closure: FnOnce(Arg) -> Return,
    Return: IntoWhoops,
{
    fn catch<HandleErrorClosure, HandleErrorClosureReturn>(
        self,
        closure: HandleErrorClosure,
    ) -> impl FnOnce(Arg) -> Whoops
    where
        HandleErrorClosure: FnOnce(&Box<dyn Error>) -> HandleErrorClosureReturn,
        HandleErrorClosureReturn: IntoWhoops,
    {
        |arg| match self(arg).into_whoops() {
            Ok(_) => Ok(()),
            Err(error) => {
                closure(&error);
                Err(error)
            }
        }
    }
}

pub trait Run<Arg> {
    fn run(self, arg: Arg);
}

impl<Closure, Arg, Return> Run<Arg> for Closure
where
    Closure: FnOnce(Arg) -> Return,
{
    fn run(self, arg: Arg) {
        _ = self(arg);
    }
}
