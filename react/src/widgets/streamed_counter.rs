use std::time::Duration;

use crate::{
    message::MessageFlow::Propagate,
    prelude::{Component, text},
    widget::Widget,
};

pub fn streamed_counter() -> Component {
    Widget::stream(
        |sender| async move {
            for i in 0.. {
                _ = sender.send(i);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        },
        |_, _| Propagate,
        |stream| match stream.current() {
            Some(i) => text(i.to_string()),
            None => text("0"),
        },
    )
}
