use ast::Node;

#[derive(Debug)]
pub struct NodeBoolean(bool);

impl NodeBoolean {
    pub fn new(b: bool) -> Self {
        NodeBoolean(b)
    }
}

impl Node for NodeBoolean {
    fn boolify(&self) -> bool {
        (self).0
    }

    fn print_ast(&self) -> String {
        (self).0.to_string()
    }

    fn debug_ast(&self) -> String {
        format!("Bool({:?})", (self).0)
    }
}
