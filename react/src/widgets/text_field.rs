use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use stdext::prelude::*;

use crate::{
    component::prelude::*, message::prelude::*, runtime::log, widget::Widget, widgets::text::text,
};

#[derive(Debug, Clone)]
struct TextField {
    buffer: String,
    cursor: usize,
    show_cursor: bool,
}

impl TextField {
    fn insert(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor += 1;
    }
    fn remove_left(&mut self) {
        if self.cursor > 0 {
            self.buffer.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }
    fn insert_str(&mut self, s: &str) {
        self.cursor += s.len();
        self.buffer.insert_str(self.cursor, s);
    }
    fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
    fn move_cursor_right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }
    // does not work
    fn remove_word_left(&mut self) {
        let space_idx = self
            .buffer
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if c == ' ' && i < self.cursor {
                    Some(i)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);
        while self.cursor > space_idx {
            self.remove_left();
        }
    }
}

impl Display for TextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut this = self.clone();
        if this.show_cursor {
            if this.cursor == this.buffer.len() {
                this.buffer += "\x1b[48;2;146;146;146m \x1b[0m";
            } else {
                if let Some(c) = this.buffer.chars().nth(this.cursor)
                    && c == '\n'
                {
                    this.insert(' ');
                }
                let color_sequence = "\x1b[48;2;146;146;146m";
                this.buffer.insert_str(self.cursor, color_sequence);
                this.buffer
                    .insert_str(self.cursor + color_sequence.len() + 1, "\x1b[0m");
            }
        }
        write!(f, "{}", this.buffer)
    }
}

pub fn text_field(initial: impl Display) -> Component {
    let initial = initial.to_string();
    Widget::stateful(
        TextField {
            cursor: initial.len(),
            buffer: initial,
            show_cursor: true,
        },
        |this, msg| {
            switch(msg).case(|event: &KeyEvent| match event.code {
                KeyCode::Enter => this.set_state(|buffer| buffer.insert('\n')),
                KeyCode::Backspace => this.set_state(|buffer| _ = buffer.remove_left()),
                KeyCode::Char(c) => this.set_state(|buffer| buffer.insert(c)),
                KeyCode::Left => this.set_state(|state| state.move_cursor_left()),
                KeyCode::Right => this.set_state(|state| state.move_cursor_right()),
                _ => {}
            });
            Intercept
        },
        |buffer| text(buffer.to_string()),
    )
}
