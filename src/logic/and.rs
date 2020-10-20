use crate::errors::ObviousError;
use crate::statements::{Evaluatable, Statements};
use core::fmt;

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct And {
    left: Box<Statements>,
    right: Box<Statements>,
}

impl And {
    #[inline(always)]
    pub fn new(left: Statements, right: Statements) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl Evaluatable for And {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError> {
        Ok(self.left.evaluate()? && self.right.evaluate()?)
    }

    #[inline(always)]
    fn evaluate_with_variables(
        &self,
        variables: &BTreeMap<String, bool>,
    ) -> Result<bool, ObviousError> {
        Ok(self.left.evaluate_with_variables(variables)?
            && self.right.evaluate_with_variables(variables)?)
    }
}

impl fmt::Display for And {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} \\wedge {})", self.left, self.right)
    }
}
