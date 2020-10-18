use obvious::table::TruthTableBuilder;
use obvious::prelude::*;

fn main() {
    let table = TruthTableBuilder::build(&["a", "b", "c"], |vars| {
        let a = vars[0];
        let b = vars[1];
        let c = vars[2];
        let first = a.implies(b);
        let second = first.implies(c);
        let third = a.implies(c);
        let statement = second.equates(third);

        statement
    });
    println!("The truth table for the statement {} is:\n{}", table.statement, table);
}
