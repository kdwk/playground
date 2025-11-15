use std::{cell::RefCell, rc::Rc};

use react::{prelude::*, widgets::timer::timer};
use stdext::prelude::*;

fn main() -> Result<()> {
    render(row([
        column([
            counter(12),
            text_field(Rc::new(RefCell::new(String::new()))),
        ]),
        column([
            text_field(Rc::new(RefCell::new(String::new()))),
            download("https://www.rust-lang.org"),
        ]),
    ]))
    // render(timer())
}
