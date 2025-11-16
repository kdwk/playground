use std::{fmt::Display, sync::Arc};

use crate::prelude::{Component, Propagate, Task, Widget, text};

pub fn download(url: impl Display + 'static) -> Component {
    let url = Arc::new(url.to_string());
    Widget::future(
        async move {
            reqwest::get(url.to_string())
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        },
        |_, _| Propagate,
        |content| match content {
            Task::Done(s) => text(s[0..100].to_string()),
            Task::Running(_) => text("Downloading"),
            Task::Err(_) => text("Error!"),
        },
    )
}
