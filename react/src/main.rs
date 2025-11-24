use std::io::Result;

use react::prelude::*;

fn main() -> Result<()> {
    render(row([
        column([counter(12), text_field("")]),
        column([text_field(""), download("https://www.rust-lang.org")]),
    ]))
}
