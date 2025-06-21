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
    /// Attempts to turn the formula into a closure `Fn(usize) -> bool`.
    /// Only works if the formula is quantifier-free and has at most one free variable.
    /// The closure does not borrow from the formula and is `'static`.
    pub fn as_closure(self) -> Result<Box<dyn Fn(usize) -> bool + 'static>, &'static str> {
        if !self.is_quantifier_free() {
            return Err("Formula contains quantifiers");
        }
        let free_vars = self.free_variables();
        if free_vars.len() > 1 {
            return Err("Formula must have at most one free variable");
        }
        let var_opt = free_vars.into_iter().next().map(|s| s.to_string());

        fn expr_to_closure(expr: crate::formulae::Expr, var: Option<String>) -> Box<dyn Fn(usize) -> i64 + 'static> {
            match expr {
                crate::formulae::Expr::Add(e1, e2) => {
                    let c1 = expr_to_closure(*e1, var.clone());
                    let c2 = expr_to_closure(*e2, var.clone());
                    Box::new(move |x| c1(x) + c2(x))
                }
                crate::formulae::Expr::Sub(e1, e2) => {
                    let c1 = expr_to_closure(*e1, var.clone());
                    let c2 = expr_to_closure(*e2, var.clone());
                    Box::new(move |x| c1(x) - c2(x))
                }
                crate::formulae::Expr::MulConst(c, e) => {
                    let ce = expr_to_closure(*e, var.clone());
                    Box::new(move |x| c * ce(x))
                }
                crate::formulae::Expr::Mod(e, m) => {
                    let ce = expr_to_closure(*e, var.clone());
                    Box::new(move |x| ce(x) % m)
                }
                crate::formulae::Expr::Var(v) => {
                    if let Some(ref var_name) = var {
                        if v == *var_name {
                            Box::new(move |x| x as i64)
                        } else {
                            // Should not happen for quantifier-free, single-variable formulas
                            Box::new(|_| 0)
                        }
                    } else {
                        // No free variable, so always 0
                        Box::new(|_| 0)
                    }
                }
                crate::formulae::Expr::Const(c) => Box::new(move |_| c),
            }
        }

        fn formula_to_closure(
            formula: Formula,
            var: Option<String>,
        ) -> Box<dyn Fn(usize) -> bool + 'static> {
            match formula {
                Formula::And(fs) => {
                    let cs: Vec<_> = fs.into_iter().map(|f| formula_to_closure(f, var.clone())).collect();
                    Box::new(move |x| cs.iter().all(|c| c(x)))
                }
                Formula::Or(fs) => {
                    let cs: Vec<_> = fs.into_iter().map(|f| formula_to_closure(f, var.clone())).collect();
                    Box::new(move |x| cs.iter().any(|c| c(x)))
                }
                Formula::Not(f) => {
                    let c = formula_to_closure(*f, var);
                    Box::new(move |x| !c(x))
                }
                Formula::Eq(e1, e2) => {
                    let c1 = expr_to_closure(*e1, var.clone());
                    let c2 = expr_to_closure(*e2, var.clone());
                    Box::new(move |x| c1(x) == c2(x))
                }
                Formula::Neq(e1, e2) => {
                    let c1 = expr_to_closure(*e1, var.clone());
                    let c2 = expr_to_closure(*e2, var.clone());
                    Box::new(move |x| c1(x) != c2(x))
                }
                Formula::Lt(e1, e2) => {
                    let c1 = expr_to_closure(*e1, var.clone());
                    let c2 = expr_to_closure(*e2, var.clone());
                    Box::new(move |x| c1(x) < c2(x))
                }
                Formula::Le(e1, e2) => {
                    let c1 = expr_to_closure(*e1, var.clone());
                    let c2 = expr_to_closure(*e2, var.clone());
                    Box::new(move |x| c1(x) <= c2(x))
                }
                Formula::Gt(e1, e2) => {
                    let c1 = expr_to_closure(*e1, var.clone());
                    let c2 = expr_to_closure(*e2, var.clone());
                    Box::new(move |x| c1(x) > c2(x))
                }
                Formula::Ge(e1, e2) => {
                    let c1 = expr_to_closure(*e1, var.clone());
                    let c2 = expr_to_closure(*e2, var.clone());
                    Box::new(move |x| c1(x) >= c2(x))
                }
                Formula::True => Box::new(|_| true),
                Formula::False => Box::new(|_| false),
                _ => panic!("Quantifiers should not be present in quantifier-free formula"),
            }
        }

        let closure = formula_to_closure(self, var_opt);
        Ok(closure)
    }




    /// Returns true if the formula contains no quantifiers (Forall or Exists).
    pub fn is_quantifier_free(&self) -> bool {
        match self {
            Formula::Forall(_, _) | Formula::Exists(_, _) => false,
            Formula::And(fs) | Formula::Or(fs) => fs.iter().all(|f| f.is_quantifier_free()),
            Formula::Not(f) => f.is_quantifier_free(),
            Formula::Eq(_, _)
            | Formula::Neq(_, _)
            | Formula::Lt(_, _)
            | Formula::Le(_, _)
            | Formula::Gt(_, _)
            | Formula::Ge(_, _)
            | Formula::True
            | Formula::False => true,
        }
    }

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
    fn test_is_quantifier_free() {
        // Quantifier-free formula: Eq
        let f1 = Formula::Eq(
            Box::new(Expr::Var("x".to_string())),
            Box::new(Expr::Const(1)),
        );
        assert!(f1.is_quantifier_free());

        // Formula with quantifier: Forall
        let f2 = Formula::Forall(
            "x".to_string(),
            Box::new(Formula::Eq(
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Const(2)),
            )),
        );
        assert!(!f2.is_quantifier_free());

        // Nested quantifier-free formula: And
        let f3 = Formula::And(vec![
            Formula::Eq(
                Box::new(Expr::Var("y".to_string())),
                Box::new(Expr::Const(3)),
            ),
            Formula::Neq(
                Box::new(Expr::Var("z".to_string())),
                Box::new(Expr::Const(4)),
            ),
        ]);
        assert!(f3.is_quantifier_free());

        // Nested formula with quantifier: Or contains Exists
        let f4 = Formula::Or(vec![
            Formula::Eq(
                Box::new(Expr::Var("a".to_string())),
                Box::new(Expr::Const(5)),
            ),
            Formula::Exists(
                "b".to_string(),
                Box::new(Formula::Eq(
                    Box::new(Expr::Var("b".to_string())),
                    Box::new(Expr::Const(6)),
                )),
            ),
        ]);
        assert!(!f4.is_quantifier_free());
    }

    #[test]
    fn test_free_variables() {
        // Simple case
        let f = Formula::Eq(
            Box::new(Expr::Var("x".to_string())),
            Box::new(Expr::Const(1)),
        );
        let free = f.free_variables();
        assert_eq!(free, ["x"].iter().cloned().collect());
        assert!(f.has_exactly_one_free_variable("x"));
        assert!(!f.has_exactly_one_free_variable("y"));

        // With quantifier
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

        // Nested quantifiers
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


    #[test]
    fn test_as_closure() {
        // Quantifier-free, one free variable
        let f = Formula::Eq(
            Box::new(Expr::Add(
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Const(2)),
            )),
            Box::new(Expr::Const(5)),
        );
        let closure = f.as_closure().expect("Should succeed");
        assert_eq!(closure(3), true);
        assert_eq!(closure(2), false);

        // Quantifier-free, no free variable
        let f2 = Formula::True;
        let closure2 = f2.as_closure().expect("Should succeed");
        assert_eq!(closure2(0), true);
        assert_eq!(closure2(42), true);

        // Not quantifier-free
        let f3 = Formula::Forall(
            "x".to_string(),
            Box::new(Formula::Eq(
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Const(1)),
            )),
        );
        assert!(f3.as_closure().is_err());

        // More than one free variable
        let f4 = Formula::Eq(
            Box::new(Expr::Add(
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Var("y".to_string())),
            )),
            Box::new(Expr::Const(5)),
        );
        assert!(f4.as_closure().is_err());
    }
}
