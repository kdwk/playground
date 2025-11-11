use crate::{
    component::prelude::*,
    prelude::{column, single_line},
};

#[inline]
pub fn text(s: impl AsRef<str> + 'static) -> Component {
    column(
        s.as_ref()
            .split("\n")
            .map(|line| single_line(line.to_string())),
    )
}
