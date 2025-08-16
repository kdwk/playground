use itertools::fold;

pub mod prelude {
    pub use super::Facts;
}

pub struct Facts {
    i: u64,
}

impl Facts {
    fn new() -> Self {
        Self { i: 0 }
    }
}

impl Iterator for Facts {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        Some((1..=self.i).into_iter().product::<u64>())
    }
}

pub mod test {
    use itertools::Itertools;

    use crate::recipe::prelude::*;

    use super::prelude::*;

    pub fn test1() {
        Facts::new().take(10).collect_vec().debug();
    }
}
