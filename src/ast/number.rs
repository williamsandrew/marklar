use ast::Node;

#[derive(Debug)]
pub struct NodeNumber(i64);

impl NodeNumber {
    pub fn new(n: i64) -> Self {
        NodeNumber(n)
    }
}

impl Node for NodeNumber {
    fn boolify(&self) -> bool {
        match (self).0 {
            0 => false,
            _ => true,
        }
    }

    fn print_ast(&self) -> String {
        (self).0.to_string()
    }

    fn debug_ast(&self) -> String {
        format!("Number({:?})", (self).0)
    }
}
