mod parallel_table;
mod solver;
mod table;

pub use parallel_table::ParallelBruteforceTruthTableBuilder;
pub use solver::BruteforceSolver;
pub use table::{BruteforceTruthTableBuilder, TruthTable};

/// converts `state` into a `Vec<bool>` containing the amounts of booleans needed
/// to represent a state of size `n`
/// TODO: Again better performance with const generics
fn usize_to_state(state: usize, n: usize) -> Vec<bool> {
    let mut res = Vec::with_capacity(n);
    for value in 0..n {
        res.push((state & (1<<value)) != 0);
    }
    res
}
