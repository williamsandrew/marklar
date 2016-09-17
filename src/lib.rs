#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod ast;

peg_file! filter("grammar.rustpeg");
use filter::expression;

#[test]
fn main() {
    assert!(expression("1==1").is_ok());
    assert!(expression("1== 1").is_ok());
    assert!(expression("1\t== 1").is_ok());

    assert!(expression("1!=2").is_ok());
    assert!(expression("1!= 2").is_ok());
    assert!(expression("1 != 2").is_ok());
}
