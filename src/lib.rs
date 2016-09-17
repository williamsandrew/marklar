#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod ast;

use filter::expression;

peg! filter(r#"
use ast::Expression;
use std::str::FromStr;

// Whitespace
__
    = [ \t]*

number -> Expression
    = n:([0-9]+ { i64::from_str(match_str).unwrap() }) __ { Expression::Number(n) }

equals -> Expression
    = l:number "==" __ r:number { Expression::Equals(Box::new(l), Box::new(r)) }

not_equals -> Expression
    = l:number "!=" __ r:number { Expression::Equals(Box::new(l), Box::new(r)) }


#[pub]
expression -> Expression
    = __ (equals / not_equals)

"#);

#[test]
fn main() {
    assert!(expression("1==1").is_ok());
    assert!(expression("1== 1").is_ok());
    assert!(expression("1\t== 1").is_ok());

    assert!(expression("1!=2").is_ok());
    assert!(expression("1!= 2").is_ok());
    assert!(expression("1 != 2").is_ok());
}
