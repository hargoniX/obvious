use crate::bruteforce::TruthTable;
use crate::errors::ObviousError;
use crate::statements::{Evaluatable, Statements};
use crate::variable::Variable;
use crate::bruteforce::usize_to_state;

use rayon::prelude::*;
use std::collections::BTreeMap;

pub struct ParallelBruteforceTruthTableBuilder {}

impl ParallelBruteforceTruthTableBuilder {
    pub fn build<'a, F>(
        names: &[&'a str],
        make_statement: F,
    ) -> Result<TruthTable, ObviousError>
    where
        F: Fn(Vec<Statements>) -> Statements,
    {
        // TODO: Once we get const generics we don't need vectors anymore for this.
        let variables: Vec<Variable> = names
            .iter()
            .map(|name| Variable::new(String::from(*name)))
            .collect();

        let statement = make_statement(
            variables
                .iter()
                .map(|variable| Statements::Variable(variable.clone()))
                .collect(),
        );

        let state_size = variables.len();
        let table: Result<BTreeMap<Vec<bool>, bool>, ObviousError> = (0..2usize
            .pow(variables.len() as u32))
            .into_par_iter()
            .map(|counter| {
                let mut values = BTreeMap::new();
                let state = usize_to_state(counter, state_size);

                for (index, variable) in variables.iter().enumerate() {
                    values.insert(variable.name.clone(), state[index]);
                }

                Ok((
                    values.values().copied().collect(),
                    statement.evaluate_with_variables(&values)?,
                ))
            })
            .collect();


        Ok(TruthTable {
            statement,
            variables: variables
                .iter()
                .map(|variable| variable.name.clone())
                .collect(),
            table: table?,
        })
    }
}
