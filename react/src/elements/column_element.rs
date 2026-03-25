use crate::prelude::{DisplayList, Element, Operation, Point, ProposedSize, Size};

pub mod prelude {
    pub use super::ColumnElement;
}

pub struct ColumnElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for ColumnElement {
    fn propose_size(&self, proposed_constraints: ProposedSize) -> ProposedSize {
        ProposedSize {
            x: proposed_constraints.x,
            y: self
                .children
                .iter()
                .map(|child| {
                    child.propose_size(ProposedSize {
                        x: proposed_constraints.x,
                        y: None,
                    }).y
                })
                .sum(),
        }.min(proposed_constraints)
    }
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let avg_child_height = (constraint.y as usize / self.children.len()) as isize;
        let mut y_offset = 0;
        for (i, child) in self.children.iter().enumerate() {
            let child_proposed_size = child.propose_size(ProposedSize { x: Some(constraint.x), y: None });
            let child_size = Size {
                x: child_proposed_size.x.unwrap_or(constraint.x).min(constraint.x),
                y: child_proposed_size.y.unwrap_or(avg_child_height).min(constraint.y)
            };
            let offset = Point {
                x: 0,
                y: y_offset as isize,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            child.draw(
                Size {
                    x: constraint.x,
                    y: child_size.y.unwrap_or(),
                },
                display_list,
            );
            display_list.0.push(Operation::SetAnchor(-offset));
            y_offset += child_height;
        }
    }
}
