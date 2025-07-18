use std::{
    default::Default,
    io::{self, Stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, read},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::mpsc::unbounded_channel;

use crate::{context::Context, input::Input};

pub mod prelude {
    pub use super::{Surface, Widget, render, run};
}

pub type Surface = Vec<Vec<char>>;

pub enum Drawable {
    Char(char),
}

pub type Instruction = ((u16, u16), Drawable);

pub(crate) struct DisplayList {
    pub(crate) size: (u16, u16),
    pub(crate) instructions: Vec<Instruction>,
}

pub trait Widget {
    fn draw(&self, context: &Context) -> DisplayList {
        self.build(context).draw(context)
    }
    fn build(&self, context: &Context) -> Box<dyn Widget>;
}

pub fn widget(w: impl Widget + 'static) -> Box<dyn Widget> {
    Box::new(w)
}

// pub fn render(
//     root: &impl Widget,
//     context: &Context,
//     last: &Surface,
//     stdout: &mut Stdout,
// ) -> io::Result<Surface> {
//     let surface = root.build(context).draw(context);
//     for (y, row) in surface.iter().enumerate() {
//         for (x, col) in row.iter().enumerate() {
// let last_col = last.get(y).and_then(|row| row.get(x));
// let should_render = if let Some(last_col) = last_col {
//     col != last_col
// } else {
//     true
// };
//             if should_render {
//                 stdout.queue(MoveTo(x as u16, y as u16))?;
//                 print!("{}", col);
//             }
//         }
//     }
//     stdout.flush()?;
//     Ok(surface)
// }

pub fn render(display_list: DisplayList, stdout: &mut Stdout) -> io::Result<()> {
    stdout.queue(Clear(ClearType::All))?;
    for (location @ (y, x), drawable) in display_list.instructions {
        if location.0 < display_list.size.0 && location.1 < display_list.size.1 {
            match drawable {
                Drawable::Char(c) => {
                    stdout.queue(MoveTo(x, y))?;
                    print!("{}", c);
                }
            }
        }
    }
    stdout.flush()?;
    Ok(())
}

pub async fn run(mut app: impl Widget) -> io::Result<()> {
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
            thread::sleep(Duration::from_millis(10));
        }
    });
    let widget = &mut app;
    let mut t = 0;
    loop {
        if let Some(Some(event)) = receiver.recv().await {
            let size = if let Ok((cols, rows)) = terminal::size() {
                (rows, cols)
            } else {
                (100, 100)
            };
            let context = Context::new();
            context.inject(Input::from(event));
            t += 1;
            render(widget.draw(&context), &mut stdout)?;
        } else {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    println!("Did render: {t} times");
    Ok(())
}
