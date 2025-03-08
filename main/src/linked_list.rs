use std::ops::Index;

use extend::ext;

#[ext]
impl<T> Vec<T> {
    #[inline]
    fn last_index(&self) -> usize {
        self.len() - 1
    }
}

struct Node<T> {
    data: T,
    next: Option<usize>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

struct LinkedList<T> {
    len: usize,
    front: Option<usize>,
    end: Option<usize>,
    nodes: Vec<Option<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            len: 0,
            front: None,
            end: None,
            nodes: vec![],
        }
    }
    fn len(&self) -> usize {
        self.len
    }
    fn push(&mut self, data: T) {
        self.nodes.push(Some(Node::new(data)));
        let index = self.nodes.last_index();
        if self.front.is_none() {
            self.front = Some(index);
        }
        if let Some(orig_last_index) = self.end {
            if let Some(node) = &mut self.nodes[orig_last_index] {
                node.next = Some(index);
                self.len += 1;
            }
        }
        self.end = Some(index);
    }
    // fn pop(&mut self) -> Option<T> {
    //     let last_node = self.nodes[self.end?];

    // }
}

// impl<T> Index<usize> for LinkedList<T> {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         assert!(index < self.len());
//         let node = self.nodes[self.end.unwrap()];
//         for i in 0..index {}
//     }
// }
