use obvious::prelude::*;
use obvious::bruteforce::BruteforceSolver;

fn main() {
    println!("Proof by bruteforce that contraposition is equivalent to implication.");
    let statement = BruteforceSolver::solve(&["a", "b"], |vars| {
        let a = vars[0];
        let b = vars[1];
        let implication = a.implies(b);
        let contraposition = b.not().implies(a.not());
        let statement = implication.equates(contraposition);

        statement
    }).unwrap();
    println!("The statement: {} is true", statement);
    println!("QED");
}
