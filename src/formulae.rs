use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    MulConst(i64, Box<Expr>),
    Mod(Box<Expr>, i64),
    Var(String),
    Const(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Formula {
    Forall(String, Box<Formula>),
    Exists(String, Box<Formula>),
    And(Vec<Formula>),
    Or(Vec<Formula>),
    Not(Box<Formula>),
    Eq(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    True,
    False,
}

impl Formula {
    /// Returns true if the formula has exactly one free variable named `t`.
    pub fn has_exactly_one_free_variable(&self, t: &str) -> bool {
        let free = self.free_variables();
        free.len() == 1 && free.contains(t)
    }

    /// Returns a set of all free variable names in the formula.
    pub fn free_variables(&self) -> HashSet<&str> {
        let mut bound = HashSet::new();
        let mut free = HashSet::new();
        self.collect_free_variables(&mut bound, &mut free);
        free
    }

    fn collect_free_variables<'a>(
        &'a self,
        bound: &mut HashSet<&'a str>,
        free: &mut HashSet<&'a str>,
    ) {
        match self {
            Formula::Forall(var, body) | Formula::Exists(var, body) => {
                bound.insert(var.as_str());
                body.collect_free_variables(bound, free);
                bound.remove(var.as_str());
            }
            Formula::And(fs) | Formula::Or(fs) => {
                for f in fs {
                    f.collect_free_variables(bound, free);
                }
            }
            Formula::Not(f) => f.collect_free_variables(bound, free),
            Formula::Eq(e1, e2)
            | Formula::Neq(e1, e2)
            | Formula::Lt(e1, e2)
            | Formula::Le(e1, e2)
            | Formula::Gt(e1, e2)
            | Formula::Ge(e1, e2) => {
                e1.collect_free_variables(bound, free);
                e2.collect_free_variables(bound, free);
            }
            Formula::True | Formula::False => {}
        }
    }
}

impl Expr {
    fn collect_free_variables<'a>(&'a self, bound: &HashSet<&'a str>, free: &mut HashSet<&'a str>) {
        match self {
            Expr::Add(e1, e2) | Expr::Sub(e1, e2) => {
                e1.collect_free_variables(bound, free);
                e2.collect_free_variables(bound, free);
            }
            Expr::MulConst(_, e) | Expr::Mod(e, _) => e.collect_free_variables(bound, free),
            Expr::Var(v) => {
                if !bound.contains(v.as_str()) {
                    free.insert(v.as_str());
                }
            }
            Expr::Const(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_variables_simple() {
        let f = Formula::Eq(
            Box::new(Expr::Var("x".to_string())),
            Box::new(Expr::Const(1)),
        );
        let free = f.free_variables();
        assert_eq!(free, ["x"].iter().cloned().collect());
        assert!(f.has_exactly_one_free_variable("x"));
        assert!(!f.has_exactly_one_free_variable("y"));
    }

    #[test]
    fn test_free_variables_quantifiers() {
        let f = Formula::Forall(
            "x".to_string(),
            Box::new(Formula::Eq(
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Var("y".to_string())),
            )),
        );
        let free = f.free_variables();
        assert_eq!(free, ["y"].iter().cloned().collect());
        assert!(f.has_exactly_one_free_variable("y"));
        assert!(!f.has_exactly_one_free_variable("x"));
    }

    #[test]
    fn test_free_variables_nested() {
        let f = Formula::Exists(
            "z".to_string(),
            Box::new(Formula::And(vec![
                Formula::Eq(
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Var("z".to_string())),
                ),
                Formula::Eq(
                    Box::new(Expr::Var("y".to_string())),
                    Box::new(Expr::Const(0)),
                ),
            ])),
        );
        let free = f.free_variables();
        assert_eq!(free, ["x", "y"].iter().cloned().collect());
        assert!(!f.has_exactly_one_free_variable("x"));
        assert!(!f.has_exactly_one_free_variable("y"));
    }
}
