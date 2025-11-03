use crate::{
    component::prelude::*,
    element::FrameExt,
    prelude::Frame,
    runtime::{go_block, wait_for},
};
use std::{
    io::{self, Write},
    thread,
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
use tokio::{
    sync::mpsc::{UnboundedSender, unbounded_channel},
    task::JoinHandle,
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

fn setup() -> (UnboundedSender<Frame>, JoinHandle<Result<()>>) {
    let (sender, mut receiver) = unbounded_channel();
    let printing_task = go_block(move || -> Result<()> {
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        while let Some(frame) = receiver.blocking_recv() {
            print_frame(frame)?;
        }
        stdout.execute(Clear(ClearType::All))?;
        disable_raw_mode()?;
        stdout.execute(LeaveAlternateScreen)?;
        Ok(())
    });
    (sender, printing_task)
}

pub fn render(widget: Component) -> Result<()> {
    let (frame_sender, mut printing_task) = setup();
    let frame = widget.borrow_mut().create_element().draw();
    frame_sender.send(frame)?;
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
                        drop(frame_sender);
                        return wait_for(&mut printing_task)?;
                    }
                    _ => {}
                }
                widget.borrow_mut().on_keypress(&event);
            }
        }
        let frame = widget.borrow_mut().create_element().draw();
        frame_sender.send(frame)?;
        thread::sleep(Duration::from_millis(10));
    }
}
