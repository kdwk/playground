use std::time::Duration;
use tokio::time::sleep;

use crate::{
    message::prelude::*,
    prelude::{Component, Task, Widget, column, counter, text, timer},
};

pub fn delayed(secs: u64) -> Component {
    Widget::future(
        async move {
            sleep(Duration::from_secs(secs)).await;
        },
        |_, _| Propagate,
        |opt| {
            column([
                match opt {
                    Task::Done(_) => text("After"),
                    Task::Running(_) => text("Before"),
                    Task::Err(_) => text("Error!"),
                },
                timer(),
            ])
        },
    )
}
