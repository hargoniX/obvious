use obvious::compile_time_assert;
use obvious::prelude::*;

fn main() {
    println!("Proof that contraposition is equivalent to implication.");
    for a in [true, false].iter() {
        for b in [true, false].iter() {
            let implication = a.implies(b);
            let contraposition = b.not().implies(a.not());
            let statement = implication.equates(contraposition);
            compile_time_assert!(statement);
        }
    }
    println!("QED");
}
