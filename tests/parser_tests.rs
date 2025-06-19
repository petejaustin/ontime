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
    // Only check top-level structure for brevity
    if let Formula::And(left, right) = f {
        assert!(matches!(*left, Formula::Eq(_, _)));
        assert!(matches!(*right, Formula::Or(_, _)));
    } else {
        panic!("Expected And at top level");
    }
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

#[test]
fn test_formula1() {
    parse_formula("(= (mod x 5) 0)");
}
