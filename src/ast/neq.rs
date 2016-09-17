use ast::Node;

#[derive(Debug)]
pub struct NodeNeq {
    left: Box<Node>,
    right: Box<Node>,
}

impl NodeNeq {
    pub fn new(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        NodeNeq {
            left: lhs,
            right: rhs,
        }
    }
}

impl Node for NodeNeq {
    fn boolify(&self) -> bool {
        self.left.boolify() != self.right.boolify()
    }

    fn print_ast(&self) -> String {
        format!("{} != {}", self.left, self.right)
    }

    fn debug_ast(&self) -> String {
        format!("Neq({:?}, {:?})", self.left, self.right)
    }
}

