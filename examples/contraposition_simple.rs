use obvious::prelude::*;

fn main() {
    println!("Proof that contraposition is equivalent to implication.");
    for a in [true, false].iter() {
        for b in [true, false].iter() {
            let a = Statements::Boolean(*a);
            let b = Statements::Boolean(*b);

            let implication = a.implies(&b);
            let contraposition = b.not().implies(&a.not());
            let statement = implication.equates(&contraposition);
            assert!(statement.evaluate().unwrap());
            println!("The substatement: {} is true", statement);
        }
    }
    println!("QED");
}
