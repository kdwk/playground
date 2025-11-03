use crate::{component::prelude::*, elements::column_element::ColumnElement, widget::prelude::*};

pub fn column(children: impl IntoIterator<Item = Component>) -> Component {
    let widgets = children.into_iter().collect::<Vec<_>>();
    Widget::elemental(widgets, propagate, |this| {
        Box::new(ColumnElement {
            children: this
                .state
                .iter()
                .map(|child| child.borrow_mut().create_element())
                .collect(),
        })
    })
}
