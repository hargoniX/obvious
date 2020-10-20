use crate::statements::{Evaluatable, Statements};
use crate::errors::ObviousError;
use core::fmt;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Or {
    left: Box<Statements>,
    right: Box<Statements>,
}

impl Or {
    #[inline(always)]
    pub fn new(left: Statements, right: Statements) -> Self {
        Self { left: Box::new(left), right: Box::new(right) }
    }
}

impl Evaluatable for Or {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError> {
        Ok(self.left.evaluate()? || self.right.evaluate()?)
    }

    #[inline(always)]
    fn evaluate_with_variables(&self, variables: &HashMap<String, bool>) -> Result<bool, ObviousError> {
        Ok(self.left.evaluate_with_variables(variables)? || self.right.evaluate_with_variables(variables)?)
    }
}

impl fmt::Display for Or {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} \\lor {})", self.left, self.right)
    }
}

