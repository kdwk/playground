pub mod prelude {
    pub use super::{SubStr};
}

pub trait SubStr {
    fn substr(&self, start: usize, end: usize) -> String;
}

impl SubStr for str {
    #[inline]
    fn substr(&self, start: usize, end: usize) -> String {
        self.chars().skip(start).take(end - start).collect()
    }
}

pub(crate) mod test {
    use crate::recipe::Log;
    use super::*;

    pub fn test1() {
        "abcdef".substr(1, 3).log();
        "abcdef".to_string().substr(1, 3).log();
    }
}