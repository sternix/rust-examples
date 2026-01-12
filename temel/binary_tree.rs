enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

use BinaryTree::*;

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

struct TreeIter<'a, T: 'a> {
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let node = match self.unvisited.pop() {
            None => return None,
            Some(n) => n,
        };

        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}

impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<'_, T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn make_node<T>(left: BinaryTree<T>, element: T, right: BinaryTree<T>) -> BinaryTree<T> {
    NonEmpty(Box::new(TreeNode {
        left,
        element,
        right,
    }))
}

fn main() {
    let subtree_l = make_node(Empty, "mecha", Empty);
    let subtree_rl = make_node(Empty, "droid", Empty);
    let subtree_r = make_node(subtree_rl, "robot", Empty);
    let tree = make_node(subtree_l, "Jaeger", subtree_r);

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }

    assert_eq!(v, ["mecha", "Jaeger", "droid", "robot"]);

    assert_eq!(
        tree.iter()
            .map(|name| format!("mega-{}", name))
            .collect::<Vec<_>>(),
        vec!["mega-mecha", "mega-Jaeger", "mega-droid", "mega-robot"]
    );
}
