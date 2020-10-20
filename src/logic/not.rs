use crate::errors::ObviousError;
use crate::statements::{Evaluatable, Statements};
use core::fmt;

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Not {
    inner: Box<Statements>,
}

impl Not {
    #[inline(always)]
    pub fn new(inner: Statements) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl Evaluatable for Not {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError> {
        Ok(!self.inner.evaluate()?)
    }

    #[inline(always)]
    fn evaluate_with_variables(
        &self,
        variables: &BTreeMap<String, bool>,
    ) -> Result<bool, ObviousError> {
        Ok(!self.inner.evaluate_with_variables(variables)?)
    }
}

impl fmt::Display for Not {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\overline{{{}}}", self.inner)
    }
}
