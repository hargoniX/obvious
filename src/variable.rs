use crate::errors::ObviousError;
use crate::statements::Evaluatable;

use core::fmt;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Evaluatable for Variable {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError> {
        Err(ObviousError::NotConstant(self.name.clone()))
    }

    #[inline(always)]
    fn evaluate_with_variables(
        &self,
        variables: &BTreeMap<String, bool>,
    ) -> Result<bool, ObviousError> {
        match variables.get(&self.name) {
            Some(value) => Ok(*value),
            None => Err(ObviousError::NotConstant(self.name.clone())),
        }
    }
}
