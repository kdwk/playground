use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    cursor::MoveTo,
    event::{self, Event, EventStream, KeyCode, KeyEvent, KeyModifiers},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use futures::StreamExt;
use tokio::task;

use crate::{elements::prelude::*, hooks::ON_KEYPRESS_CBS, widget::Widget};

pub mod prelude {
    pub use super::render;
}

fn print_frame(frame: Frame) -> Result<()> {
    let mut stdout = io::stdout();
    for row in frame {
        stdout.execute(MoveTo(0, 0))?;
        stdout.write(row.iter().collect::<String>().as_bytes())?;
        stdout.write(b"\n")?;
    }
    Ok(())
}

fn setup(widget: Rc<RefCell<dyn Widget>>) -> Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    _ = begin_listen_keypress(widget);
    Ok(())
}

fn teardown() -> Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All))?;
    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}

pub async fn render(widget: Rc<RefCell<dyn Widget>>) -> Result<()> {
    setup(widget.clone())?;
    let mut element_tree = widget.borrow_mut().create_element();
    print_frame(element_tree.draw())?;
    loop {
        if event::poll(Duration::default())? {
            let event = event::read()?;
            widget.borrow_mut().on_keypress(&event);
        }
        element_tree = widget.borrow_mut().create_element();
        print_frame(element_tree.draw())?;
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

async fn begin_listen_keypress(widget: Rc<RefCell<dyn Widget>>) {
    task::spawn_local(async move {
        let mut events = EventStream::new();
        while let Some(event) = events.next().await {
            if let Ok(event) = event {
                // widget.borrow_mut().on_keypress(&event);
            }
        }
    });
}
