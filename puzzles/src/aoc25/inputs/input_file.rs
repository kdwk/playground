use std::{error::Error, fmt::Display};

use documents::{Create, Document};

pub fn input_file(day: impl Display) -> Result<Document, Box<dyn Error>> {
    Document::at_path(
        format!("/Users/kdwk/Projects/personal/playground/puzzles/src/aoc25/inputs/{day}.txt"),
        "input",
        Create::No,
    )
}
