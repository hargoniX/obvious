use crate::errors::ObviousError;
use crate::statements::{Evaluatable, Statements};
use crate::variable::Variable;

use core::fmt;
use std::collections::HashMap;

pub struct BruteforceTruthTableBuilder {}

impl BruteforceTruthTableBuilder {
    pub fn build<'a, T, F>(
        names: &[&'a str],
        make_statement: F,
    ) -> Result<TruthTable<T>, ObviousError>
    where
        F: Fn(Vec<Statements>) -> T,
        T: Evaluatable,
    {
        let mut table: HashMap<Vec<bool>, bool> = HashMap::new();
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

        let mut variable_values: HashMap<String, bool> = variables
            .iter()
            .map(|variable| (variable.name.clone(), false))
            .collect();

        table.insert(
            variable_values.values().copied().collect(),
            statement.evaluate_with_variables(&variable_values)?,
        );

        for counter in 1..2usize.pow((variables.len()) as u32) {
            for index in 0..variables.len() {
                if counter % 2usize.pow(index as u32) == 0 {
                    variable_values.insert(
                        variables[index].name.clone(),
                        !variable_values[&variables[index].name],
                    );
                }
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
pub struct TruthTable<S: Evaluatable> {
    pub statement: S,
    pub variables: Vec<String>,
    pub table: HashMap<Vec<bool>, bool>,
}

impl<S: Evaluatable> fmt::Display for TruthTable<S> {
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
