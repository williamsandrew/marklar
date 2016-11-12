#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod ast;

peg_file! filter("grammar.rustpeg");
use filter::machine;

#[test]
fn main() {
    assert!(machine("1==1").is_ok());
    assert!(machine("1== 1").is_ok());
    assert!(machine("1\t== 1").is_ok());

    assert!(machine("1+1").is_ok());
    assert!(machine("2 == 1 + 1").is_ok());
    assert!(machine("1!=2").is_ok());
    assert!(machine("1!= 2").is_ok());
    assert!(machine("1\t!= 2").is_ok());

    assert!(machine("1 + 1 == 2").is_ok());
    assert!(machine("1 + 1 + 2").is_ok());

    assert!(machine("1 * 2").is_ok());
    assert!(machine("1 / 2").is_ok());

    assert!(machine("1 * 2 + 1").is_ok());
}

#[test]
fn primitives() {
    assert!(machine("1").is_ok());
    assert!(machine("true").is_ok());
    assert!(machine("false").is_ok());
}

#[test]
fn grouping() {
    assert!(machine("(false)").is_ok());
    assert!(machine("(1==1) + 1").is_ok());
    assert!(machine("((1==1) + 1) - 2").is_ok());
}
