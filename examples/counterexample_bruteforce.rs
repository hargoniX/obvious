use obvious::bruteforce::BruteforceSolver;

fn main() {
    println!("Proof by bruteforce that implication is not always true");

    let result = BruteforceSolver::solve(&["a", "b"], |vars| {
        let a = vars[0].clone();
        let b = vars[1].clone();

        a.implies(&b)
    });

    assert!(result.is_err());
    println!("The counterexample is {:?}", result);
    println!("QED");
}
