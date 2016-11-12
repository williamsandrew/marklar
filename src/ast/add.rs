use ast::Node;

#[derive(Debug)]
pub struct NodeAdd {
    left: Box<Node>,
    right: Box<Node>,
}

impl NodeAdd {
    pub fn new(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        NodeAdd {
            left: lhs,
            right: rhs,
        }
    }
}

impl Node for NodeAdd {
    fn boolify(&self) -> bool {
        true
    }

    fn print_ast(&self) -> String {
        format!("{} + {}", self.left, self.right)
    }

    fn debug_ast(&self) -> String {
        format!("Neq({:?}, {:?})", self.left, self.right)
    }
}

