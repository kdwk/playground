pub mod prelude {
    pub use super::{Element, Frame, FrameExt, ensure_same_height, ensure_same_width};
}

pub type Frame = Vec<Vec<char>>;

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
        self.iter().map(Vec::len).max().unwrap_or(0)
    }
    fn align_width(&mut self) {
        let max_width = self.max_width();
        for row in self.iter_mut() {
            let diff = max_width - row.len();
            row.append(&mut vec![' '; diff]);
        }
    }
    fn expand_to_height(&mut self, target: usize) {
        let width = self.max_width();
        let diff = target - self.height();
        if diff > 0 {
            self.append(&mut vec![vec![' '; width]; diff]);
        }
    }
}

pub trait Element {
    fn draw(&self) -> Frame;
}

// export function rect(frames: Frame[]): Frame[] {
//     const framesSameHeight = ensureSameLength(frames, []);
//     return framesSameHeight.map(frame => ensureSameLength(frame, " "));
// }

// export function frameSize(frame: Frame): {width: number; height: number} {
//     return {
//         width: (frame.at(0) ?? []).length,
//         height: frame.length
//     };
// }

pub fn ensure_same_height(frames: &mut Vec<Frame>) {
    let max_height = frames.iter().map(FrameExt::height).max().unwrap_or(0);
    for frame in frames {
        frame.align_width();
        frame.expand_to_height(max_height);
    }
}

pub fn ensure_same_width(frames: &mut Vec<Frame>) {
    for frame in frames {
        frame.align_width();
    }
}
