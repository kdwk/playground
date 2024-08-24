use chrono::prelude::*;

pub mod prelude {
    pub use super::{Dimensions, Fruit, Fruit::Apple, Fruit::Banana, Fruit::Orange, Ripeness};
}

pub enum Fruit {
    Apple {
        date_of_picking: DateTime<Local>,
        dimensions: Dimensions,
    },
    Orange {
        date_of_picking: DateTime<Local>,
        dimensions: Dimensions,
    },
    Banana {
        date_of_picking: DateTime<Local>,
        dimensions: Dimensions,
    },
}

impl Fruit {
    fn ripeness(&self) -> Ripeness {
        match self {
            Fruit::Apple {
                date_of_picking,
                dimensions: _,
            }
            | Fruit::Orange {
                date_of_picking,
                dimensions: _,
            }
            | Fruit::Banana {
                date_of_picking,
                dimensions: _,
            } => {
                let days_since_picking = (Local::now() - date_of_picking).num_days();
                if days_since_picking < 2 {
                    Ripeness::Unripe
                } else if days_since_picking < 5 {
                    Ripeness::Ripe
                } else {
                    Ripeness::Overripe
                }
            }
        }
    }
}

pub enum Ripeness {
    Unripe,
    Ripe,
    Overripe,
}

pub struct Dimensions {
    x: f32,
    y: f32,
    z: f32,
}

impl Dimensions {
    pub fn new(x: impl Into<f32>, y: impl Into<f32>, z: impl Into<f32>) -> Self {
        Dimensions {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}
