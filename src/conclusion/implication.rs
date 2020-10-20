use crate::statements::{Evaluatable, Statements};
use crate::errors::ObviousError;
use core::fmt;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Implication {
    left: Box<Statements>,
    right: Box<Statements>,
}

impl Implication {
    #[inline(always)]
    pub fn new(left: Statements, right: Statements) -> Self {
        Self { left: Box::new(left), right: Box::new(right) }
    }
}

impl Evaluatable for Implication {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError>  {
        // A \Rightarrow B is equivalent to \bar{A} \lor B
        self.left.not().or(self.right.as_ref()).evaluate()
    }

    #[inline(always)]
    fn evaluate_with_variables(&self, variables: &HashMap<String, bool>) -> Result<bool, ObviousError> {
        // A \Rightarrow B is equivalent to \bar{A} \lor B
        self.left.not().or(self.right.as_ref()).evaluate_with_variables(variables)
    }
}

impl fmt::Display for Implication {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} \\Rightarrow {})", self.left, self.right)
    }
}

