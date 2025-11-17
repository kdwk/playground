use std::{cell::RefCell, rc::Rc};

use react::{
    prelude::*,
    widgets::{streamed_counter::streamed_counter, timer::timer},
};
use stdext::prelude::*;

fn main() -> Result<()> {
    render(row([
        column([counter(12), text_field("")]),
        column([text_field(""), download("https://www.rust-lang.org")]),
    ]))
    // render(timer())
    // render(streamed_counter())
}
