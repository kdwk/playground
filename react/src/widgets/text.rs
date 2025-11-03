use crate::{
    component::prelude::*,
    prelude::{row, single_char},
    widgets::column::column,
};

pub fn text(s: impl AsRef<str> + 'static) -> Component {
    column(
        s.as_ref()
            .split("\n")
            .map(|line| row(line.chars().map(|c| single_char(c)))),
    )
}
