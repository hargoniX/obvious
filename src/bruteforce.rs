use crate::logic::LogicError;
use crate::traits::Statement;

use core::fmt;

pub struct BruteforceSolver {}

impl BruteforceSolver {
    pub fn solve<'a, T, F>(names: &[&'a str], make_statement: F) -> Result<T, LogicError<'a>>
    where
        F: Fn(&[BruteforceVariable<'a>]) -> T,
        T: Statement,
    {
        // TODO: Once we get const generics we don't need vectors anymore for this.
        let mut variables: Vec<BruteforceVariable<'a>> = names
            .iter()
            .map(|name| BruteforceVariable::new(name))
            .collect();
        let mut statement = make_statement(&variables);

        if !statement.evaluate() {
            return Err(LogicError::CounterExample(
                variables
                    .iter()
                    .map(|variable| (variable.name, variable.inner))
                    .collect(),
            ));
        }

        for counter in 1..2usize.pow((variables.len()) as u32) {
            for index in 0..variables.len() {
                if counter % 2usize.pow(index as u32) == 0 {
                    variables[index].inner = !variables[index].inner;
                }
            }
            statement = make_statement(&variables);

            if !statement.evaluate() {
                return Err(LogicError::CounterExample(
                    variables
                        .iter()
                        .map(|variable| (variable.name, variable.inner))
                        .collect(),
                ));
            }
        }

        Ok(statement)
    }
}

#[derive(Copy, Clone)]
pub struct BruteforceVariable<'a> {
    inner: bool,
    name: &'a str,
}

impl<'a> BruteforceVariable<'a> {
    fn new(name: &'a str) -> Self {
        Self { inner: false, name }
    }
}

impl<'a> fmt::Debug for BruteforceVariable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.name, self.inner)
    }
}

impl<'a> fmt::Display for BruteforceVariable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> Statement for BruteforceVariable<'a> {
    #[inline(always)]
    fn evaluate(&self) -> bool {
        self.inner
    }
}
