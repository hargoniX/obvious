use obvious::bruteforce::ParallelBruteforceTruthTableBuilder;

fn main() {
    let table = ParallelBruteforceTruthTableBuilder::build(&["a", "b", "c"], |vars| {
        let a = vars[0].clone();
        let b = vars[1].clone();
        let c = vars[2].clone();
        let first = a.implies(&b);
        let second = first.implies(&c);
        let third = a.implies(&c);
        let statement = second.equates(&third);

        statement
    })
    .unwrap();
    println!(
        "The truth table for the statement {} is:\n{}",
        table.statement, table
    );
}
