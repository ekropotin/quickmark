use tree_sitter::{Node, Tree};

#[derive(Copy, Clone, Debug)]
pub enum TraversalOrder {
        PreOrder,
        PostOrder,
}

#[derive(Debug)]
pub struct TreeSitterWalker<'a> {
    pub order: TraversalOrder,
    pub tree: &'a Tree
}

impl<'a> TreeSitterWalker<'a> {
    pub fn new(tree: &'a Tree) -> Self {
        Self {
            tree,
            order: TraversalOrder::PreOrder,
        }
    }

    pub fn with_order(tree: &'a Tree, order: TraversalOrder) -> Self {
        Self { tree, order }
    }

    pub fn walk(&self, mut callback: impl FnMut(Node)) {
        let root = self.tree.root_node();
        match self.order {
            TraversalOrder::PreOrder => self.walk_pre_order(root, &mut callback),
            TraversalOrder::PostOrder => self.walk_post_order(root, &mut callback),
        }

    }

    #[allow(clippy::only_used_in_recursion)]
    fn walk_pre_order(&self, node: Node, callback: &mut impl FnMut(Node)) {
        callback(node);
        for child in node.children(&mut node.walk()) {
            self.walk_pre_order(child, callback);
        }
    }
    #[allow(clippy::only_used_in_recursion)]
    fn walk_post_order(&self, node: Node, callback: &mut impl FnMut(Node)) {
        for child in node.children(&mut node.walk()) {
            self.walk_post_order(child, callback);
        }
        callback(node);
    }
}
