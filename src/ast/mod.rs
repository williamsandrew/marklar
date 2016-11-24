use std::fmt;

pub mod eq;
pub mod neq;
pub mod add;
pub mod sub;
pub mod not;
pub mod div;
pub mod mult;
pub mod boolean;
pub mod number;

pub use ast::boolean::*;
pub use ast::number::*;

#[derive(Debug)]
pub enum Expression {
    Number(NodeNumber),
    Boolean(NodeBoolean),

    Not(Box<Node>),

    Add(Box<Node>),
    Subtract(Box<Node>),

    Multiply(Box<Node>),
    Divide(Box<Node>),

    Equals(Box<Node>),
    NotEquals(Box<Node>),
}


pub trait Node {
    fn reduce(self) -> Result<Box<Expression>, &'static str>;
    // fn print_ast(&self) -> String;
    fn debug_ast(&self) -> String;
}

pub trait NodePrimitive {
    fn boolify(&self) -> bool;
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debug_ast())
    }
}

pub struct Reducers { }

impl Reducers {
    pub fn reduce_no_bool(expr: Box<Expression>, msg: &'static str) -> Result<Box<Expression>, &'static str> {
        match *expr {
            Expression::Boolean(_) => { Err(msg) },
            Expression::Number(e) => { Ok(box Expression::Number(e)) },
            Expression::Not(e) => { e.reduce() },
            Expression::Add(e) => { e.reduce() },
            Expression::Subtract(e) => { e.reduce() },
            Expression::Multiply(e) => { e.reduce() },
            Expression::Divide(e) => { e.reduce() },
            Expression::Equals(e) => { e.reduce() },
            Expression::NotEquals(e) => { e.reduce() }
        }
    }

    pub fn reduce(expr: Box<Expression>) -> Result<Box<Expression>, &'static str> {
        match *expr {
            Expression::Boolean(e) => { Ok(box Expression::Boolean(e)) },
            Expression::Number(e) => { Ok(box Expression::Number(e)) },
            Expression::Not(e) => { e.reduce() },
            Expression::Add(e) => { e.reduce() },
            Expression::Subtract(e) => { e.reduce() },
            Expression::Multiply(e) => { e.reduce() },
            Expression::Divide(e) => { e.reduce() },
            Expression::Equals(e) => { e.reduce() },
            Expression::NotEquals(e) => { e.reduce() }
        }
    }
}

#[derive(Debug)]
pub struct Machine {
    pub expression: Box<Expression>
}

impl Machine {
    pub fn new(expr: Box<Expression>) -> Self {
        Machine {
            expression: expr
        }
    }

    // pub fn run(&mut self) -> bool {
    //     self.expression.boolify()
    // }
}
