use crate::errors::ObviousError;
use crate::statements::{Evaluatable, Statements};
use crate::variable::Variable;
use crate::bruteforce::usize_to_state;

use core::fmt;
use std::collections::BTreeMap;

pub struct BruteforceTruthTableBuilder {}

impl BruteforceTruthTableBuilder {
    pub fn build<'a, F>(
        names: &[&'a str],
        make_statement: F,
    ) -> Result<TruthTable, ObviousError>
    where
        F: Fn(Vec<Statements>) -> Statements,
    {
        let mut table: BTreeMap<Vec<bool>, bool> = BTreeMap::new();
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

        let mut variable_values: BTreeMap<String, bool> = variables
            .iter()
            .map(|variable| (variable.name.clone(), false))
            .collect();

        let state_size = variables.len();
        for counter in 0..2usize.pow((state_size) as u32) {
            let state = usize_to_state(counter, state_size);

            for (index, variable) in variables.iter().enumerate() {
                variable_values.insert(variable.name.clone(), state[index]);
            }

            table.insert(
                variable_values.values().copied().collect(),
                statement.evaluate_with_variables(&variable_values)?,
            );
        }

        Ok(TruthTable {
            statement,
            variables: variables
                .iter()
                .map(|variable| variable.name.clone())
                .collect(),
            table,
        })
    }
}

// TODO: This type can be implemented much better with const generics as well
pub struct TruthTable {
    pub statement: Statements,
    pub variables: Vec<String>,
    pub table: BTreeMap<Vec<bool>, bool>,
}

impl fmt::Display for TruthTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut table_format = String::from("|");
        for _ in 0..self.variables.len() + 1 {
            table_format.push_str("c|");
        }
        write!(
            f,
            "\\begin{{tabular}}{{ {} }}\n    \\hline\n    ",
            table_format
        )?;

        for variable in self.variables.iter() {
            write!(f, "{} & ", variable)?;
        }

        writeln!(f, "{} \\\\", self.statement)?;
        writeln!(f, "    \\hline")?;

        for (values, result) in self.table.iter() {
            write!(f, "    ")?;
            for value in values {
                write!(f, "{} & ", value)?;
            }

            writeln!(f, "{} \\\\", result)?;
            writeln!(f, "    \\hline")?;
        }

        write!(f, "\\end{{tabular}}")
    }
}
