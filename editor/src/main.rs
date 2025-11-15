use std::{cell::RefCell, rc::Rc};

use documents::prelude::*;
use react::prelude::*;

use crate::app::app;

mod app;

fn main() {
    let mut args = std::env::args();
    _ = args.next();
    let filename = args.next().unwrap_or("".to_string());
    with(
        &[Document::at_path(filename, "doc", Create::OnlyIfNotExists)],
        |d| {
            render(app(d["doc"].clone()))?;
            Ok(())
        },
    );
}
