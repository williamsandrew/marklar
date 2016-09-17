#[derive(Debug)]
pub enum Expression {
    Number(i64),
    Equals(Box<Expression>, Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
}
