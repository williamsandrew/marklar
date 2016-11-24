use ast::{Node,Expression,Reducers};

#[derive(Debug)]
pub struct NodeMult {
    left: Box<Node>,
    right: Box<Node>,
}

impl NodeMult {
    pub fn new(lhs: Box<Node>, rhs: Box<Node>) -> Self {
        NodeMult {
            left: lhs,
            right: rhs,
        }
    }
}

impl Node for NodeMult {
    fn debug_ast(&self) -> String {
        format!("Mult({:?}, {:?})", self.left, self.right)
    }

    fn reduce(self) -> Result<Box<Expression>, &'static str> {
        let left_side = try!(Reducers::reduce_no_bool(self.left, "cannot multiply boolean"));
        let right_side = try!(Reducers::reduce_no_bool(self.right, "cannot multiply boolean"));

        match (*left_side, *right_side) {
            (Expression::Number(l), Expression::Number(r)) => { Ok(Expression::Number(l * r)) },
            _ => { panic!("shouldn't be here"); }
        }
    }
}
