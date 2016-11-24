use ast::{Node,NodeBoolean,Expression,Reducers};

#[derive(Debug)]
pub struct NodeNeq {
    left: Expression,
    right: Expression,
}

impl NodeNeq {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        NodeNeq {
            left: lhs,
            right: rhs,
        }
    }
}

impl Node for NodeNeq {
    // fn boolify(&self) -> bool {
    //     self.left.boolify() != self.right.boolify()
    // }

    // fn print_ast(&self) -> String {
    //     format!("{} != {}", self.left, self.right)
    // }
    fn reduce(self) -> Result<Box<Expression>, &'static str> {
        let left_side = try!(Reducers::reduce(self.left));
        let right_side = try!(Reducers::reduce(self.right));

        match (left_side, right_side) {
            (Expression::Boolean(l), Expression::Boolean(r)) => {
                // Ok(Expression::Boolean(NodeBoolean::from(l != r)))
                x_bool!(l != r)
            },
            (Expression::Number(l), Expression::Number(r)) => {
                // Ok(Expression::Boolean(NodeBoolean::from(l != r)))
                x_bool!(l != r)
            },
            (Expression::Number(l), Expression::Boolean(r)) => {
                // Ok(Expression::Boolean(NodeBoolean::from(false)))
                x_bool!(false)
            },
            (Expression::Boolean(l), Expression::Number(r)) => {
                // Ok(Expression::Boolean(NodeBoolean::from(false)))
                x_bool!(false)
            },
            _ => { panic!("shouldn't be here"); }
        }
    }

    fn debug_ast(&self) -> String {
        format!("Neq({:?}, {:?})", self.left, self.right)
    }
}

