use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub mod prelude {
    pub use super::{Assertable, AssertionError};
}

#[derive(Debug)]
pub struct AssertionError<Expected: Debug, Found: Debug>(Expected, Found);

impl<Expected: Debug, Found: Debug> Display for AssertionError<Expected, Found> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Assertion error: expected {:?}, got {:?}",
            self.0, self.1
        )
    }
}

impl<Expected: Debug, Found: Debug> Error for AssertionError<Expected, Found> {}

pub trait Assertable {
    fn should_be<Value: PartialEq<Self> + Debug>(
        self,
        value: Value,
    ) -> Result<Self, AssertionError<Value, Self>>
    where
        Self: Debug + Sized;
    fn must_be(self, value: impl PartialEq<Self> + Debug) -> Self;
}

impl<T: PartialEq + Debug> Assertable for T {
    fn should_be<Value: PartialEq<Self> + Debug>(
        self,
        value: Value,
    ) -> Result<Self, AssertionError<Value, Self>> {
        if value == self {
            Ok(self)
        } else {
            Err(AssertionError(value, self))
        }
    }
    fn must_be(self, value: impl PartialEq<Self> + Debug) -> Self {
        assert_eq!(value, self);
        self
    }
}

pub mod test {
    use super::*;

    pub fn test1() {
        (1 + 2).should_be(3).unwrap();
        "bubble".to_string().must_be("bubbles");
    }
}
