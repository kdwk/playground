use std::{fmt::Display, sync::Arc};

use crate::{component::Component, runtime::go, widget::Widget, widgets::text::text};

pub fn download(url: impl Display + 'static) -> Component {
    let url = Arc::new(url.to_string());
    let content = go(async move {
        reqwest::get(url.to_string())
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    });
    Widget::future(
        content,
        |_, _| {},
        |content| match content {
            Some(s) => text(s[0..100].to_string()),
            None => text("Downloading"),
        },
    )
}
