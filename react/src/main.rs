use std::io::Result;

use react::prelude::*;

fn main() -> Result<()> {
    render(row([
        column([counter(12), text_field("").0]),
        column([text_field("").0, download("https://www.rust-lang.org")]),
    ]))
}
