use crate::{component::prelude::*, prelude::CharElement, widget::Widget};

pub fn single_char(c: char) -> Component {
    Widget::elemental((), |_, _| (), move |_| Box::new(CharElement { c }))
}
