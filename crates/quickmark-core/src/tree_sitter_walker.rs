use tree_sitter::{Node, Tree};

#[derive(Copy, Clone, Debug)]
pub enum TraversalOrder {
    PreOrder,
    PostOrder,
}

#[derive(Debug)]
pub struct TreeSitterWalker<'a> {
    pub order: TraversalOrder,
    pub tree: &'a Tree,
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
        let mut cursor = self.tree.walk();
        match self.order {
            TraversalOrder::PreOrder => self.walk_pre_order(&mut cursor, &mut callback),
            TraversalOrder::PostOrder => self.walk_post_order(&mut cursor, &mut callback),
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn walk_pre_order(&self, cursor: &mut tree_sitter::TreeCursor, callback: &mut impl FnMut(Node)) {
        let node = cursor.node();
        callback(node);
        
        if cursor.goto_first_child() {
            loop {
                self.walk_pre_order(cursor, callback);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }
    
    #[allow(clippy::only_used_in_recursion)]
    fn walk_post_order(&self, cursor: &mut tree_sitter::TreeCursor, callback: &mut impl FnMut(Node)) {
        if cursor.goto_first_child() {
            loop {
                self.walk_post_order(cursor, callback);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
        
        let node = cursor.node();
        callback(node);
    }
}
