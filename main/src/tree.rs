enum BinaryTree<'a, T> {
    Leaf(Option<T>),
    Node(&'a Self, T, &'a Self),
}

use BinaryTree::*;

impl<'a, T: PartialOrd> BinaryTree<'a, T> {
    fn contains(&self, target: Option<&T>) -> bool {
        match self {
            Leaf(val) => target == val.as_ref(),
            Node(tree1, val, tree2) => {
                target == Some(val)
                    || if Some(val) < target {
                        tree1.contains(target)
                    } else {
                        tree2.contains(target)
                    }
            }
        }
    }
    fn insert(&'a mut self, item: T) {
        // match self {
        //     Leaf(None) => *self = Leaf(Some(item)),
        //     Leaf(Some(val)) => {
        //         if item < *val {
        //             *self = Node(&Leaf(Some(item)), *val, &Leaf(None));
        //         } else if item > *val {
        //             *self = Node(&Leaf(None), *val, &Leaf(Some(item)))
        //         }
        //     }
        //     Node(binary_tree, _, binary_tree1) => todo!(),
        // }
    }
}
