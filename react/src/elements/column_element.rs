use crate::{
    prelude::{Constraint2, DisplayList, Element, Operation, Point, ProposedSize, Size},
    utils::{Constraint::Pixel, IteratorOptionIsizeExt, OptionConstraintExt, Realize},
};

pub mod prelude {
    pub use super::ColumnElement;
}

pub struct ColumnElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for ColumnElement {
    fn propose_size(&self, proposed_constraints: Constraint2) -> ProposedSize {
        ProposedSize {
            x: proposed_constraints.propose_as_pixels().x,
            y: self
                .children
                .iter()
                .map(|child| {
                    child
                        .propose_size(Constraint2 {
                            x: proposed_constraints.x,
                            y: None,
                        })
                        .y
                })
                .realize(proposed_constraints.propose_as_pixels().y)
                .map(|constraint| constraint
                    .expect_pixel_or_none("Unable to realize flex layout. There must be a pixel-constrained parent to any element proposing flex size.")
                )
                .sum_or_none()
                .and_then(|pix| Some(Pixel(pix)))
            ,
        }
    }
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList) {
        let avg_child_height = constraint
            .y
            .and_then(|y| Some((y as usize / self.children.len()) as isize));
        let mut y_offset = 0;
        let children_heights = self.children.iter().map(|child| child.propose_size(Constraint2 { x: constraint.x, y: None }).y).realize(constraint.propose_as_pixels().y).map(|constraint| constraint
                    .expect_pixel_or_none("Unable to realize flex layout. There must be a pixel-constrained parent to any element proposing flex size.")
                );
        for (child, height) in self.children.iter().zip(children_heights) {
            let child_size = Constraint2 {
                x: constraint.x,
                y: child_proposed_size
                    .y
                    .unwrap_or(avg_child_height)
                    .min((constraint.y - y_offset).max(0)),
            };
            let offset = Point {
                x: 0,
                y: y_offset as isize,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            child.draw(child_size, display_list);
            display_list.0.push(Operation::SetAnchor(-offset));
            y_offset += child_size.y;
        }
    }
}
