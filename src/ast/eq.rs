use ast::{Node,NodeBoolean,Expression,Reducers};

pub struct NodeEq {
    left: Expression,
    right: Expression,
}

impl NodeEq {
    pub fn new(lhs: Expression, rhs: Expression) -> Self {
        NodeEq {
            left: lhs,
            right: rhs,
        }
    }
}

impl Node for NodeEq {
    // fn boolify(&self) -> bool {
    //     self.left.boolify() == self.right.boolify()
    // }

    // fn print_ast(&self) -> String {
    //     format!("{} == {}", self.left, self.right)
    // }
    fn reduce(self) -> Result<Box<Expression>, &'static str> {
        let left_side = try!(Reducers::reduce(self.left));
        let right_side = try!(Reducers::reduce(self.right));

        match (left_side, right_side) {
            (Expression::Boolean(l), Expression::Boolean(r)) => { Ok(Expression::Boolean(NodeBoolean::from(l == r))) },
            (Expression::Number(l), Expression::Number(r)) => {  Ok(Expression::Boolean(NodeBoolean::from(l == r))) }
            (Expression::Number(l), Expression::Boolean(r)) => { Ok(Expression::Boolean(NodeBoolean::from(false))) },
            (Expression::Boolean(l), Expression::Number(r)) => { Ok(Expression::Boolean(NodeBoolean::from(false))) },
            _ => { panic!("shouldn't be here"); }
        }
    }

    fn debug_ast(&self) -> String {
        format!("Eq({:?}, {:?})", self.left, self.right)
    }
}
