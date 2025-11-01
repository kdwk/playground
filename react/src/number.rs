use std::{cell::RefCell, rc::Rc};

use react::{component::Component, widget::Widget};

use react::elements::number_element::NumberElement;

pub fn number(i: i32) -> Rc<RefCell<dyn Component>> {
    Widget::elemental((), |_, _| (), move |_| Box::new(NumberElement { i }))
}
