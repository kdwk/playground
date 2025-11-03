use crate::{component::prelude::*, prelude::RowElement, widget::prelude::*};

pub fn row(children: impl IntoIterator<Item = Component>) -> Component {
    let widgets = children.into_iter().collect::<Vec<_>>();
    Widget::elemental(widgets, propagate, move |this| {
        Box::new(RowElement {
            children: this
                .state
                .iter()
                .map(|child| child.borrow_mut().create_element())
                .collect(),
        })
    })
}
