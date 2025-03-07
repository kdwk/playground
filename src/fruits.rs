use crate::object::prelude::*;
use chrono::prelude::*;
use object_derive::{Enum, Object};
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use super::{Dimensions, Fruit, Fruit::Apple, Fruit::Banana, Fruit::Orange, Ripeness};
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub fn ripeness(&self) -> Ripeness {
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
    pub fn is_ripe(&self) -> bool {
        match self.ripeness() {
            Ripeness::Ripe | Ripeness::Overripe => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Ripeness {
    Unripe,
    Ripe,
    Overripe,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dimensions {
    x: f64,
    y: f64,
    z: f64,
}

impl Dimensions {
    const ZERO: Self = Self::new_const(0., 0., 0.);
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Dimensions {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
    pub const fn new_const(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
