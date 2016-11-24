macro_rules! x_bool {
    ($x:expr) => {
        Ok(Expression::Boolean(NodeBoolean::from($x)))
    }
}
