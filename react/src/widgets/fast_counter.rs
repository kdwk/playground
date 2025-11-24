use stdext::prelude::switch;

use crate::{
    message::MessageFlow::Propagate,
    prelude::{Component, text},
    render::Tick,
    widget::Widget,
};

pub fn fast_counter() -> Component {
    Widget::stateful(
        0,
        |this, msg| {
            switch(msg).case(|&Tick(from_start)| {
                this.set_state(|tick| {
                    *tick = ((from_start.as_millis() as f64 / 1000.0) * 12.0).round() as u64
                })
            });
            Propagate
        },
        |tick| text(tick.to_string()),
    )
}
