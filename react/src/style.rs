use crate::{
    prelude::{Component, Pipe, Pipeline, ProposedSize, SizedElement, Widget},
    widget::propagate,
};

pub mod prelude {
    pub use super::Style;
}

pub trait Style {
    fn style(self, styles: impl Pipeline<Component, Component>) -> Component;
}

impl Style for Component {
    fn style(self, styles: impl Pipeline<Component, Component>) -> Component {
        self.pipe(styles)
    }
}

pub fn sized(x: Option<isize>, y: Option<isize>) -> impl Fn(Component) -> Component {
    move |child| {
        Widget::elemental(
            child,
            |this, msg| this.state.borrow_mut().on_message(msg),
            move |this| {
                let (did_child_rebuild, child) = this.state.borrow_mut().create_element();
                (did_child_rebuild, Box::new(SizedElement { size: ProposedSize { x, y }, child }))
            },
        )
    }
}
