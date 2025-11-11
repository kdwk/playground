use crate::{
    component::prelude::*,
    element::FrameExt,
    message::{handle_messages, send},
    prelude::Frame,
    runtime::{go_block, wait_for},
};
use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
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
    pub use super::{Tick, render};
}

pub struct Tick(pub Duration);

fn print_frame(frame: Frame) -> Result<()> {
    let mut stdout = io::stdout();
    stdout.queue(Clear(ClearType::All))?;
    for row_index in 0..frame.height() {
        if row_index >= u16::MAX as usize {
            break;
        }
        stdout.queue(MoveTo(0, row_index as u16))?;
        print!("{}", frame[row_index]);
    }
    stdout.flush()?;
    Ok(())
}

fn setup() -> (UnboundedSender<Frame>, JoinHandle<Result<()>>) {
    let (sender, mut receiver) = unbounded_channel();
    let printing_task = go_block(move || -> Result<()> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(Hide)?;
        while let Some(frame) = receiver.blocking_recv() {
            print_frame(frame)?;
        }
        stdout.execute(Show)?;
        stdout.execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    });
    (sender, printing_task)
}

pub fn render(widget: Component) -> Result<()> {
    let (frame_sender, mut printing_task) = setup();
    let start = Instant::now();
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
                    _ => send(event),
                }
            }
        }
        send(Tick(start.elapsed()));
        handle_messages(|msg| widget.borrow_mut().on_message(msg));
        let frame = widget.borrow_mut().create_element().draw();
        frame_sender.send(frame)?;
        thread::sleep(Duration::from_millis(10));
    }
}
