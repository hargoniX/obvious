use crate::statements::{Evaluatable, Statements};
use crate::errors::ObviousError;
use crate::variable::Variable;

use std::collections::HashMap;

pub struct BruteforceSolver {}

impl BruteforceSolver {
    pub fn solve<'a, T, F>(names: &[&'a str], make_statement: F) -> Result<T, ObviousError>
    where
        F: Fn(Vec<Statements>) -> T,
        T: Evaluatable,
    {
        // TODO: Once we get const generics we don't need vectors anymore for this.
        let variables: Vec<Variable> = names
            .iter()
            .map(|name| Variable::new(String::from(*name)))
            .collect();
        let statement = make_statement(variables.iter().map(|variable| Statements::Variable(variable.clone())).collect());

        let mut variable_values: HashMap<String, bool> = variables.iter().map(|variable| (variable.name.clone(), false)).collect();

        if !statement.evaluate_with_variables(&variable_values)? {
            return Err(ObviousError::CounterExample(variable_values))
        }

        for counter in 1..2usize.pow((variables.len()) as u32) {
            for index in 0..variables.len() {
                if counter % 2usize.pow(index as u32) == 0 {
                    variable_values.insert(variables[index].name.clone(), !variable_values[&variables[index].name]);
                }
            }

            if !statement.evaluate_with_variables(&variable_values)? {
                return Err(ObviousError::CounterExample(variable_values))
            }
        }

        Ok(statement)
    }
}

