use crate::prelude::{DisplayList, Element, Operation, Point, ProposedSize, Size};

pub mod prelude {
    pub use super::RowElement;
}

pub struct RowElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for RowElement {
    fn propose_size(&self, proposed_constraints: ProposedSize) -> ProposedSize {
        ProposedSize {
            y: proposed_constraints.y,
            x: self
                .children
                .iter()
                .map(|child| {
                    child
                        .propose_size(ProposedSize {
                            x: None,
                            y: proposed_constraints.y,
                        })
                        .x
                })
                .sum(),
        }
        .min(proposed_constraints)
    }
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        // let child_width = constraint.x as usize / self.children.len();
        // let mut x_offset = 0;
        // for child in &self.children {
        //     let offset = Point {
        //         x: x_offset as isize,
        //         y: 0,
        //     };
        //     display_list.0.push(Operation::SetAnchor(offset));
        //     child.draw(
        //         Size {
        //             x: child_width as isize,
        //             y: constraint.y,
        //         },
        //         display_list,
        //     );
        //     display_list.0.push(Operation::SetAnchor(-offset));
        //     x_offset += child_width;
        // }
        let avg_child_width = (constraint.x as usize / self.children.len()) as isize;
        let mut x_offset = 0;
        for child in &self.children {
            let child_proposed_size = child.propose_size(ProposedSize {
                y: Some(constraint.y),
                x: None,
            });
            let child_size = Size {
                x: child_proposed_size
                    .x
                    .unwrap_or(avg_child_width)
                    .min((constraint.x - x_offset).max(0)),
                y: child_proposed_size
                    .y
                    .unwrap_or(constraint.y)
                    .min(constraint.y),
            };
            let offset = Point {
                x: x_offset as isize,
                y: 0,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            child.draw(child_size, display_list);
            display_list.0.push(Operation::SetAnchor(-offset));
            x_offset += child_size.x;
        }
    }
}
