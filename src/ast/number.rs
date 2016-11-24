use std::cmp::PartialEq;
use std::convert::From;
use std::ops::{Add,Sub,Mul,Div,Neg};

use ast::{Node,NodePrimitive,Expression,Reducers};

#[derive(Debug,Clone,Copy)]
pub struct NodeNumber(i64);

impl NodeNumber {
    pub fn new(n: i64) -> Self {
        NodeNumber(n)
    }
}

impl NodePrimitive for NodeNumber {
    fn boolify(&self) -> bool {
        match (self).0 {
            0 => false,
            _ => true,
        }
    }
}

impl Node for NodeNumber {

    // fn print_ast(&self) -> String {
    //     (self).0.to_string()
    // }
    fn reduce(self) -> Result<Box<Expression>, &'static str> {
        Ok(Expression::Number(self.clone()))
    }

    fn debug_ast(&self) -> String {
        format!("Number({:?})", (self).0)
    }
}

impl PartialEq for NodeNumber {
    fn eq(&self, other: &NodeNumber) -> bool {
        self.0 == other.0
    }
}

impl Add for NodeNumber {
    type Output = Self;

    fn add(self, other: NodeNumber) -> Self {
        Self::new(self.0 + other.0)
    }
}

impl Sub for NodeNumber {
    type Output = Self;

    fn sub(self, other: NodeNumber) -> Self {
        Self::new(self.0 - other.0)
    }
}

impl Mul for NodeNumber {
    type Output = Self;

    fn mul(self, other: NodeNumber) -> Self {
        Self::new(self.0 * other.0)
    }
}

impl Div for NodeNumber {
    type Output = Self;

    fn div(self, other: NodeNumber) -> Self {
        Self::new(self.0 / other.0)
    }
}
