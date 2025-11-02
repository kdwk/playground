use std::{fmt::Display, time::Duration};

// use stdext::prelude::*;
use tokio::time::sleep;

use crate::{component::Component, runtime::go, widget::Widget, widgets::text::text};

struct AlwaysRebuild<T>(T);
impl<T> PartialEq for AlwaysRebuild<T> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

pub fn delayed() -> Component {
    // let timer = go(async { sleep(Duration::from_secs(1)) });
    // Widget::stateful(
    //     AlwaysRebuild((timer, original.to_string(), change_to.to_string())),
    //     |_, _| {},
    //     move |AlwaysRebuild((timer, original, change_to))| {
    //         if timer.is_finished() {
    //             text(change_to.clone())
    //         } else {
    //             text(original.clone())
    //         }
    //     },
    // )
    let timer = go(sleep(Duration::from_secs(1)));
    Widget::future(
        timer,
        |_, _| {},
        move |opt| match opt {
            Some(_) => text("After"),
            None => text("Before"),
        },
    )
}
