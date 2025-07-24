use std::fmt::Display;
use documents::prelude::*;

pub(crate) fn log(message: impl Display) {
    with(&[Document::at_path("log.txt", "log", Create::OnlyIfNotExists)],
         |mut d| {
             d["log"].append((message.to_string() + "\n").as_bytes())?;
             Ok(())
         });
}