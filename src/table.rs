use crate::traits::Statement;
use crate::bruteforce::BruteforceVariable;

use std::collections::HashMap;
use core::fmt;

pub struct TruthTableBuilder {}

impl TruthTableBuilder {
    pub fn build<'a, T, F>(names: &[&'a str], make_statement: F) -> TruthTable<'a, T>
    where
        F: Fn(&[BruteforceVariable<'a>]) -> T,
        T: Statement,
    {
        let mut table: HashMap<Vec<bool>, bool> = HashMap::new();
        // TODO: Once we get const generics we don't need vectors anymore for this.
        let mut variables: Vec<BruteforceVariable<'a>> = names
            .iter()
            .map(|name| BruteforceVariable::new(name))
            .collect();
        let mut statement = make_statement(&variables);

        table.insert(variables.iter().map(|variable| variable.inner).collect(), statement.evaluate());

        for counter in 1..2usize.pow((variables.len()) as u32) {
            for index in 0..variables.len() {
                if counter % 2usize.pow(index as u32) == 0 {
                    variables[index].inner = !variables[index].inner;
                }
            }
            statement = make_statement(&variables);
            table.insert(variables.iter().map(|variable| variable.inner).collect(), statement.evaluate());
        }

        TruthTable {
            statement,
            variables: variables.iter().map(|variable| variable.name).collect(),
            table
        }
    }

}

// TODO: This type can be implemented much better with const generics as well
pub struct TruthTable<'a, S: Statement> {
    pub statement: S,
    pub variables: Vec<&'a str>,
    pub table: HashMap<Vec<bool>, bool>
}

impl<'a, S: Statement> fmt::Display for TruthTable<'a, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut table_format = String::from("|");
        for _ in 0..self.variables.len() + 1 {
            table_format.push_str("c|");
        }
        write!(f, "\\begin{{tabular}}{{ {} }}\n    \\hline\n    ", table_format)?;

        for variable in self.variables.iter() {
            write!(f, "{} & ", variable)?;
        }

        write!(f, "{} \\\\\n", self.statement)?;
        write!(f, "    \\hline\n")?;

        for (values, result) in self.table.iter() {
            write!(f, "    ")?;
            for value in values {
                write!(f, "{} & ", value)?;
            }

            write!(f, "{} \\\\\n", result)?;
            write!(f, "    \\hline\n")?;
        }

        write!(f, "\\end{{tabular}}")
    }
}
