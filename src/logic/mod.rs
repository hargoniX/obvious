mod or;
mod and;
mod not;

use crate::statements::Evaluatable;
use crate::errors::ObviousError;

use std::collections::HashMap;

pub use or::Or;
pub use and::And;
pub use not::Not;

impl Evaluatable for bool {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError>  {
        Ok(*self)
    }

    #[inline(always)]
    fn evaluate_with_variables(&self, _variables: &HashMap<String, bool>) -> Result<bool, ObviousError> {
        Ok(*self)
    }
}

impl Evaluatable for &bool {
    #[inline(always)]
    fn evaluate(&self) -> Result<bool, ObviousError> {
        Ok(**self)
    }

    #[inline(always)]
    fn evaluate_with_variables(&self, _variables: &HashMap<String, bool>) -> Result<bool, ObviousError> {
        Ok(**self)
    }
}
