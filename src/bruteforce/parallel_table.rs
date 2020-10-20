use crate::statements::{Evaluatable, Statements};
use crate::variable::Variable;
use crate::errors::ObviousError;
use crate::bruteforce::TruthTable;

use std::collections::HashMap;
use rayon::prelude::*;

pub struct ParallelBruteforceTruthTableBuilder {}

impl ParallelBruteforceTruthTableBuilder {
    pub fn build<'a, T, F>(names: &[&'a str], make_statement: F) -> Result<TruthTable<T>, ObviousError>
    where
        F: Fn(Vec<Statements>) -> T,
        T: Evaluatable + Sync,
    {
        let mut table: HashMap<Vec<bool>, bool> = HashMap::new();
        // TODO: Once we get const generics we don't need vectors anymore for this.
        let variables: Vec<Variable> = names
            .iter()
            .map(|name| Variable::new(String::from(*name)))
            .collect();

        let statement = make_statement(variables.iter().map(|variable| Statements::Variable(variable.clone())).collect());

        let variable_values: HashMap<String, bool> = variables.iter().map(|variable| (variable.name.clone(), false)).collect();

        table.insert(variable_values.values().map(|val| *val).collect(), statement.evaluate_with_variables(&variable_values)?);

        let results: Result<HashMap<Vec<bool>, bool>, ObviousError> = (1..2usize.pow((variables.len()) as u32)).into_par_iter().map(|counter| {
            let mut values = variable_values.clone();
            for index in 0..variables.len() {
                if counter % 2usize.pow(index as u32) == 0 {
                    values.insert(variables[index].name.clone(), !values[&variables[index].name]);
                }
            }
            Ok((values.values().map(|val| *val).collect(), statement.evaluate_with_variables(&values)?))
        }).collect();

        table.extend(results?);


        Ok(TruthTable {
            statement,
            variables: variables.iter().map(|variable| variable.name.clone()).collect(),
            table
        })
    }

}
