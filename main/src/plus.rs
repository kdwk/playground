pub trait Plus<T> {
    fn plus<It: IntoIterator<Item = T>>(self, other: It) -> Self;
}

impl<T, Ty: Extend<T>> Plus<T> for Ty {
    #[inline]
    fn plus<It: IntoIterator<Item = T>>(mut self, other: It) -> Self {
        self.extend(other);
        self
    }
}

pub mod test {
    use crate::{link, map, recipe::Log};

    use super::*;
    pub fn test1() {
        vec![1, 2, 3].plus([4, 5]).log();
        "abc".to_string().plus("def".chars()).log();
        map! {
            "a" => 1,
            "b" => 2,
        }
        .plus([("c", 3), ("d", 4)])
        .log();
        link![0].plus([]).log();
    }
}
