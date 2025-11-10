use std::f64::consts::PI;

use crossterm::event::{KeyCode, KeyEvent};
use stdext::prelude::AnythingExt;

use crate::{
    prelude::{Component, column, text},
    render::Tick,
    widget::prelude::*,
};

pub fn animated_char() -> Component {
    Widget::stateful(
        0,
        |this, msg| {
            msg.case::<Tick>(|_| this.set_state(|num_ticks| *num_ticks += 1))
                .case::<KeyEvent>(|event| match event.code {
                    KeyCode::Char(' ') => this.set_state(|num_ticks| *num_ticks = 0),
                    _ => {}
                });
        },
        |&num_ticks| {
            let c = ((num_ticks as f64 - 90.0) * PI / 180.0).sin();
            let index = std::cmp::max(0, std::cmp::min((c * 10.0 + 10.0).round() as i64, 19));
            let s = (0..20).map(|i| if i == index { "⚪️" } else { " " });
            column([text(s.collect::<String>()), text(index.to_string())])
        },
    )
}
