use std::cmp::PartialEq;
use std::convert::From;
use ast::{Node,NodeNumber,NodePrimitive,Expression,Reducers};

#[derive(Debug,Clone,Copy)]
pub struct NodeBoolean(bool);

impl NodeBoolean {
    pub fn new(b: bool) -> Self {
        NodeBoolean(b)
    }
}

impl NodePrimitive for NodeBoolean {
    fn boolify(&self) -> bool {
        (self).0
    }
}

impl Node for NodeBoolean {
    // fn print_ast(&self) -> String {
    //     (self).0.to_string()
    // }
    fn reduce(self) -> Result<Box<Expression>, &'static str> {
        Ok(Expression::Boolean(self.clone()))
    }

    fn debug_ast(&self) -> String {
        format!("Bool({:?})", (self).0)
    }
}

impl PartialEq for NodeBoolean {
    fn eq(&self, other: &NodeBoolean) -> bool {
        self.0 == other.0
    }
}

impl From<bool> for NodeBoolean {
    fn from(b: bool) -> Self {
        NodeBoolean::new(b)
    }
}

impl From<NodeNumber> for NodeBoolean {
    fn from(num: NodeNumber) -> Self {
        NodeBoolean::new(num.boolify())
    }
}
