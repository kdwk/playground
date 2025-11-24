use stdext::prelude::*;

use crate::{
    message::MessageFlow::Propagate,
    prelude::{Component, text},
    render::Tick,
    widget::Widget,
};

pub fn timer() -> Component {
    Widget::stateful(
        0,
        |this, msg| {
            switch(msg).case(|&Tick(duration)| {
                let secs = duration.as_secs();
                if secs != this.state {
                    this.set_state(|state| *state = secs);
                }
            });
            Propagate
        },
        |&num_secs| text(num_secs.to_string()),
    )
}
