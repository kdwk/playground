pub mod prelude {
    pub use super::{Element, Frame, FrameExt};
}

pub type Frame = Vec<String>;

pub trait FrameExt {
    fn height(&self) -> usize;
    fn first_width(&self) -> usize;
    fn max_width(&self) -> usize;
    fn align_width(&mut self);
    fn expand_to_height(&mut self, target: usize);
}

impl FrameExt for Frame {
    fn height(&self) -> usize {
        self.len()
    }
    fn first_width(&self) -> usize {
        self.get(0).and_then(|row| Some(row.len())).unwrap_or(0)
    }
    fn max_width(&self) -> usize {
        self.iter().map(String::len).max().unwrap_or(0)
    }
    fn align_width(&mut self) {
        let max_width = self.max_width();
        for row in self.iter_mut() {
            let diff = max_width - row.len();
            *row += &(std::iter::repeat_n(' ', diff).collect::<String>());
        }
    }
    fn expand_to_height(&mut self, target: usize) {
        let width = self.first_width();
        let diff = target - self.height();
        if diff > 0 {
            for _ in 0..diff {
                self.push(std::iter::repeat_n(' ', width).collect::<String>());
            }
        }
    }
}

pub trait Element: Send {
    fn draw(&self) -> Frame;
}
