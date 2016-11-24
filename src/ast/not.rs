use ast::{Node,NodeBoolean,Expression,Reducers};
use ast::NodePrimitive;

#[derive(Debug)]
pub struct NodeNot {
    left: Expression,
}

impl NodeNot {
    pub fn new(lhs: Expression) -> Self {
        NodeNot {
            left: lhs,
        }
    }
}

impl Node for NodeNot {
    // fn boolify(&self) -> bool {
    //     true
    // }

    // fn print_ast(&self) -> String {
    //     format!("{}", self.left)
    // }
    fn reduce(self) -> Result<Box<Expression>, &'static str> {
        let left_side = try!(Reducers::reduce(self.left));

        match left_side {
            Expression::Boolean(op) => {
                Ok(Expression::Boolean(NodeBoolean::from(!op.boolify())))
            },
            Expression::Number(op) => {
                Ok(Expression::Boolean(NodeBoolean::from(!op.boolify())))
            },
            _ => { panic!("shouldn't be here"); }
        }
    }

    fn debug_ast(&self) -> String {
        format!("Not({:?}", self.left)
    }
}
