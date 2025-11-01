use crate::{component::prelude::*, element::FrameExt, prelude::Frame};
use std::{
    io::{self, Write},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

pub mod prelude {
    pub use super::render;
}

fn print_frame(frame: Frame) -> Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(Clear(ClearType::All))?;
    for row_index in 0..frame.height() {
        if row_index >= u16::MAX as usize {
            break;
        }
        for (col_index, c) in frame[row_index].iter().enumerate() {
            stdout.queue(MoveTo(col_index as u16, row_index as u16))?;
            print!("{c}");
        }
        // stdout.write(frame[row_index].iter().collect::<String>().as_bytes())?;
    }
    stdout.flush()?;
    Ok(())
}

fn setup(widget: Component) -> Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    // _ = begin_listen_keypress(widget);
    Ok(())
}

fn teardown() -> Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All))?;
    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}

pub async fn render(widget: Component) -> Result<()> {
    setup(widget.clone())?;
    let element_tree = widget.borrow_mut().create_element().draw();
    print_frame(element_tree)?;
    loop {
        if event::poll(Duration::default())? {
            let event = event::read()?;
            if let Event::Key(
                event @ KeyEvent {
                    code, modifiers, ..
                },
            ) = event
            {
                match (modifiers, code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        teardown()?;
                        return Ok(());
                    }
                    _ => {}
                }
                widget.borrow_mut().on_keypress(&event);
            }
        }
        let element_tree = widget.borrow_mut().create_element().draw();
        print_frame(element_tree)?;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

fn should_quit(event: &Event) -> bool {
    if let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event
    {
        match (modifiers, code) {
            (&KeyModifiers::CONTROL, &KeyCode::Char('c')) => true,
            _ => false,
        }
    } else {
        false
    }
}
