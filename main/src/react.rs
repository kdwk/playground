use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Stdout, Write},
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, read},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::mpsc::unbounded_channel;

use crate::map;

pub mod prelude {
    pub use super::test;
}

pub struct Context {
    input: Event,
    size: (u16, u16),
}

type Surface = Vec<Vec<char>>;

pub trait Element {
    fn draw(&self, context: &Context) -> Surface {
        self.build(context).draw(context)
    }
    fn build(&self, context: &Context) -> impl Element;
}

#[derive(Default)]
pub struct SingleChar {
    pub c: char,
}

impl Element for SingleChar {
    fn draw(&self, _context: &Context) -> Surface {
        vec![vec![self.c]]
    }
    fn build(&self, _context: &Context) -> impl Element {
        panic!("Unreachable: called build on SingleChar");
        Self { c: self.c }
    }
}

pub fn render(
    root: &impl Element,
    context: &Context,
    last: &Surface,
    stdout: &mut Stdout,
) -> io::Result<Surface> {
    let surface = root.build(context).draw(context);
    for (y, row) in surface.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let last_col = last.get(y).and_then(|row| row.get(x));
            let should_render = if let Some(last_col) = last_col {
                col != last_col
            } else {
                true
            };
            if should_render {
                stdout.queue(MoveTo(x as u16, y as u16))?;
                print!("{}", col);
            }
        }
    }
    stdout.flush()?;
    Ok(surface)
}

pub async fn run(app: &impl Element) -> io::Result<()> {
    let (sender, mut receiver) = unbounded_channel();
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    tokio::task::spawn_blocking(move || {
        loop {
            if let Ok(event) = read() {
                if let Event::Key(event) = event {
                    if let KeyCode::Char('q') = event.code {
                        break;
                    }
                }
                sender.send(Some(event)).unwrap();
            }
        }
    });
    let last = &mut vec![vec![]];
    loop {
        if let Some(Some(event)) = receiver.recv().await {
            let size = if let Ok((cols, rows)) = terminal::size() {
                (rows, cols)
            } else {
                (100, 100)
            };
            let context = Context { input: event, size };
            *last = render(app, &context, last, &mut stdout)?;
        } else {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub mod test {
    use super::*;

    struct DynamicChar;

    impl Element for DynamicChar {
        fn build(&self, context: &Context) -> impl Element {
            if let Event::Key(event) = context.input {
                if let KeyCode::Char(c) = event.code {
                    return SingleChar { c };
                }
            }
            SingleChar::default()
        }
    }

    pub async fn test() {
        run(&DynamicChar).await.unwrap();
    }
}
