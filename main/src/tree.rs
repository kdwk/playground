enum Tree<'a, T: Eq> {
    Leaf(T),
    Node(&'a Self, T, &'a Self),
}

impl<'a, T: Eq> Tree<'a, T> {
    fn contains(&self, target: &T) -> bool {
        match self {
            Tree::Leaf(val) => target == val,
            Tree::Node(tree1, val, tree2) => {
                target == val || tree1.contains(target) || tree2.contains(target)
            }
        }
    }
}

impl<'a, T: Ord> Tree<'a, T> {
    fn binary_search(&self, target: &T) -> bool {
        match self {
            Tree::Leaf(val) => target == val,
            Tree::Node(tree1, val, tree2) => {
                target == val
                    || if val < target {
                        tree1.binary_search(target)
                    } else {
                        tree2.binary_search(target)
                    }
            }
        }
    }
}
