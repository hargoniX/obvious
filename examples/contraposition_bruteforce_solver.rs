use obvious::bruteforce::BruteforceSolver;

fn main() {
    println!("Proof by bruteforce that contraposition is equivalent to implication.");
    let statement = BruteforceSolver::solve(&["a", "b"], |vars| {
        let a = vars[0].clone();
        let b = vars[1].clone();
        let implication = a.implies(&b);
        let contraposition = b.not().implies(&a.not());
        let statement = implication.equates(&contraposition);

        statement
    })
    .unwrap();
    println!("The statement: {} is true", statement);
    println!("QED");
}
