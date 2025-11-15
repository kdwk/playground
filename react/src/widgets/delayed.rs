use std::time::Duration;
use tokio::time::sleep;

use crate::{
    message::prelude::*,
    prelude::{Component, Widget, column, counter, go, text},
};

pub fn delayed() -> Component {
    let timer = go(async {
        sleep(Duration::from_secs(3)).await;
        0
    });
    Widget::future(
        timer,
        |_, _| Propagate,
        |opt| {
            column([
                match opt {
                    Some(_) => text("After"),
                    None => text("Before"),
                },
                counter(1),
            ])
        },
    )
}
