use core::fmt;

use crate::traits::Statement;

#[derive(Debug, Clone, Copy)]
pub struct Implication<L: Statement, R: Statement> {
    left: L,
    right: R,
}

impl<L: Statement, R: Statement> Implication<L, R> {
    #[inline(always)]
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L: Statement, R: Statement> Statement for Implication<L, R> {
    #[inline(always)]
    fn evaluate(&self) -> bool {
        // A \Rightarrow B is equivalent to \bar{A} \lor B
        self.left.not().or(self.right).evaluate()
    }
}

impl<L: Statement, R: Statement> fmt::Display for Implication<L, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} \\Rightarrow {})", self.left, self.right)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Equivalence<L: Statement, R: Statement> {
    left: L,
    right: R,
}

impl<L: Statement, R: Statement> Equivalence<L, R> {
    #[inline(always)]
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L: Statement, R: Statement> Statement for Equivalence<L, R> {
    #[inline(always)]
    fn evaluate(&self) -> bool {
        // A \Leftrightarrow B is equivalent to (A \Rightarrow B) \wedge (B \Rightarrow A)
        self.left
            .implies(self.right)
            .and(self.right.implies(self.left))
            .evaluate()
    }
}

impl<L: Statement, R: Statement> fmt::Display for Equivalence<L, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} \\Leftrightarrow {})", self.left, self.right)
    }
}
