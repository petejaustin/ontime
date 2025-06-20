use ontime::formulae::{Expr, Formula};
use ontime::parser::formula::FormulaParser;

fn parse_formula(input: &str) -> Formula {
    FormulaParser::new().parse(input).expect("parse failed")
}

#[test]
fn test_parse_simple_comparison() {
    let f = parse_formula("(= x 1)");
    assert_eq!(
        f,
        Formula::Eq(
            Box::new(Expr::Var("x".to_string())),
            Box::new(Expr::Const(1))
        )
    );
}

#[test]
fn test_parse_and_or() {
let f = parse_formula("(and (= x 1) (or (= y 2) (not (= z 3))))");
let expected = Formula::And(vec![
    Formula::Eq(
        Box::new(Expr::Var("x".to_string())),
        Box::new(Expr::Const(1))
    ),
    Formula::Or(vec![
        Formula::Eq(
            Box::new(Expr::Var("y".to_string())),
            Box::new(Expr::Const(2))
        ),
        Formula::Not(Box::new(
            Formula::Eq(
                Box::new(Expr::Var("z".to_string())),
                Box::new(Expr::Const(3))
            )
        ))
    ])
]);
assert_eq!(f, expected);
}

#[test]
fn test_parse_forall_exists() {
    let f = parse_formula("(forall x (exists y (= x y)))");
    if let Formula::Forall(x, inner) = f {
        assert_eq!(x, "x");
        if let Formula::Exists(y, inner2) = *inner {
            assert_eq!(y, "y");
            assert!(matches!(*inner2, Formula::Eq(_, _)));
        } else {
            panic!("Expected Exists");
        }
    } else {
        panic!("Expected Forall");
    }
}

