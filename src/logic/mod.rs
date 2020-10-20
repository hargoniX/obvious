mod and;
mod not;
mod or;

use crate::errors::ObviousError;
use crate::statements::Evaluatable;

use std::collections::HashMap;

pub use and::And;
pub use not::Not;
pub use or::Or;

impl Evaluatable for bool {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError> {
        Ok(*self)
    }

    #[inline(always)]
    fn evaluate_with_variables(
        &self,
        _variables: &HashMap<String, bool>,
    ) -> Result<bool, ObviousError> {
        Ok(*self)
    }
}

impl Evaluatable for &bool {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError> {
        Ok(**self)
    }

    #[inline(always)]
    fn evaluate_with_variables(
        &self,
        _variables: &HashMap<String, bool>,
    ) -> Result<bool, ObviousError> {
        Ok(**self)
    }
}
