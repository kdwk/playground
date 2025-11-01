use crate::{component::prelude::*, prelude::RowElement, widget::Widget};

pub fn row(children: impl IntoIterator<Item = Component>) -> Component {
    let widgets = children.into_iter().collect::<Vec<_>>();
    Widget::elemental(
        (),
        |_, _| (),
        move |_| {
            Box::new(RowElement {
                children: widgets
                    .iter()
                    .map(|child| child.borrow_mut().create_element())
                    .collect(),
            })
        },
    )
}
