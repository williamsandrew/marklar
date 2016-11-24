use ast::{Node,Expression,Reducers};

#[derive(Debug)]
pub struct NodeDiv {
    left: Expression,
    right: Expression,
}

impl NodeDiv {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        NodeDiv {
            left: lhs,
            right: rhs,
        }
    }
}

impl Node for NodeDiv {
    // fn boolify(&self) -> bool {
    //     true
    // }

    // fn print_ast(&self) -> String {
    //     format!("{} / {}", self.left, self.right)
    // }
    fn reduce(self) -> Result<Box<Expression>, &'static str> {
        let left_side = try!(Reducers::reduce_no_bool(self.left, "cannot divide boolean"));
        let right_side = try!(Reducers::reduce_no_bool(self.right, "cannot divide boolean"));

        match (left_side, right_side) {
            (Expression::Number(l), Expression::Number(r)) => { Ok(Expression::Number(l / r)) },
            _ => { panic!("shouldn't be here"); }
        }
    }

    fn debug_ast(&self) -> String {
        format!("Div({:?}, {:?})", self.left, self.right)
    }
}
