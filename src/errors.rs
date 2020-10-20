use core::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ObviousError {
    /// Provides a counter example in form of a list of variables for which the statement
    /// which returned this error is not correct
    CounterExample(HashMap<String, bool>),
    /// Thrown if a logical statement that was tried to evaluated is not constant, i.e.
    /// it contains a variable that has not been given a value for this evaluation. The associated
    /// &str contains the names of said variable.
    NotConstant(String)
}

impl fmt::Display for ObviousError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObviousError::CounterExample(variables) => {
                write!(f, "CounterExample: [ ")?;
                for variable in variables.iter() {
                    write!(f, "{} = {}, ", variable.0, variable.1)?;
                }
                write!(f, " ]")
            }
            ObviousError::NotConstant(variable) => write!(f, "The variable '{}' has not been given a value", variable)
        }
    }
}
