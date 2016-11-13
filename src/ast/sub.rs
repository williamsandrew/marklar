use ast::Node;

#[derive(Debug)]
pub struct NodeSub {
    left: Box<Node>,
    right: Box<Node>,
}

impl NodeSub {
    pub fn new(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        NodeSub {
            left: lhs,
            right: rhs,
        }
    }
}

impl Node for NodeSub {
    fn boolify(&self) -> bool {
        true
    }

    fn print_ast(&self) -> String {
        format!("{} - {}", self.left, self.right)
    }

    fn debug_ast(&self) -> String {
        format!("Sub({:?}, {:?})", self.left, self.right)
    }
}
