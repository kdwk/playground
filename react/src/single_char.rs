use crate::{
    context::Context,
    widget::{DisplayList, Drawable, prelude::*, widget},
};

#[derive(Default)]
pub struct SingleChar {
    pub c: char,
}

impl Widget for SingleChar {
    fn draw(&self, _context: &Context) -> DisplayList {
        DisplayList {
            size: (1, 1),
            instructions: vec![((0, 0), Drawable::Char(self.c))],
        }
    }
    fn build(&self, _context: &Context) -> Box<dyn Widget> {
        panic!("Unreachable: called build on SingleChar")
    }
}
