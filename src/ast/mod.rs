// use std::ops::Boxed;
use std::fmt;

pub mod eq;
pub mod neq;
pub mod number;
pub mod add;
pub mod sub;
pub mod boolean;

pub trait Node {
    // fn reduce(&self) -> Atom;
    fn boolify(&self) -> bool;
    fn print_ast(&self) -> String;
    fn debug_ast(&self) -> String;
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debug_ast())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print_ast())
    }
}


#[derive(Debug)]
pub struct Machine {
    pub expression: Box<Node>
}

impl Machine {
    pub fn new(expr: Box<Node>) -> Self {
        Machine {
            expression: expr
        }
    }

    pub fn run(&mut self) -> bool {
        self.expression.boolify()
    }
}
