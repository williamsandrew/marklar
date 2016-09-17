use ast::Node;
use std::fmt;

pub struct NodeEq {
    left: Box<Node>,
    right: Box<Node>,
}

impl NodeEq {
    pub fn new(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        NodeEq {
            left: lhs,
            right: rhs,
        }
    }
}

impl Node for NodeEq {
    fn boolify(&self) -> bool {
        self.left.boolify() == self.right.boolify()
    }

    fn print_ast(&self) -> String {
        format!("{} == {}", self.left, self.right)
    }

    fn debug_ast(&self) -> String {
        format!("Eq({:?}, {:?})", self.left, self.right)
    }
}
