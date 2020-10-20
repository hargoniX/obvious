use crate::statements::{Evaluatable, Statements};
use crate::errors::ObviousError;
use core::fmt;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Equivalence {
    left: Box<Statements>,
    right: Box<Statements>,
}

impl Equivalence {
    #[inline(always)]
    pub fn new(left: Statements, right: Statements) -> Self {
        Self { left: Box::new(left), right: Box::new(right) }
    }
}

impl Evaluatable for Equivalence {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError> {
        // A \Leftrightarrow B is equivalent to (A \Rightarrow B) \wedge (B \Rightarrow A)
        self.left
            .implies(self.right.as_ref())
            .and(&self.right.implies(self.left.as_ref()))
            .evaluate()
    }

    #[inline(always)]
    fn evaluate_with_variables(&self, variables: &HashMap<String, bool>) -> Result<bool, ObviousError> {
        self.left
            .implies(self.right.as_ref())
            .and(&self.right.implies(self.left.as_ref()))
            .evaluate_with_variables(variables)
    }
}

impl fmt::Display for Equivalence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} \\Leftrightarrow {})", self.left, self.right)
    }
}
