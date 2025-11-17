use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use documents::prelude::*;
use react::prelude::*;
use stdext::prelude::*;

pub fn app(file: Document) -> Component {
    Widget::stateful(
        file.content().unwrap_or("".to_string()),
        move |this, msg| {
            switch(msg)
                .case({
                    let mut file = file.clone();
                    move |event: &KeyEvent| match (event.modifiers, event.code) {
                        (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                            _ = file.replace_with(this.state.as_bytes());
                            Intercept
                        }
                        _ => Propagate,
                    }
                })
                .default(|| Propagate)
        },
        |buffer| text_field(buffer.clone()),
    )
}
