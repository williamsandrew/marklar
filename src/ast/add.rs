use ast::{Node,Expression,Reducers};

#[derive(Debug)]
pub struct NodeAdd {
    left: Expression,
    right: Expression,
}

impl NodeAdd {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        NodeAdd {
            left: lhs,
            right: rhs,
        }
    }
}

impl Node for NodeAdd {
    // fn boolify(&self) -> bool {
    //     true
    // }

    // fn print_ast(&self) -> String {
    //     format!("{} + {}", self.left, self.right)
    // }
    fn reduce(self) -> Result<Box<Expression>, &'static str> {
        let left_side = try!(Reducers::reduce_no_bool(self.left, "cannot add boolean"));
        let right_side = try!(Reducers::reduce_no_bool(self.right, "cannot add boolean"));

        match (left_side, right_side) {
            (Expression::Number(l), Expression::Number(r)) => { Ok(Expression::Number(l + r)) },
            _ => { panic!("shouldn't be here"); }
        }
    }

    fn debug_ast(&self) -> String {
        format!("Add({:?}, {:?})", self.left, self.right)
    }
}
