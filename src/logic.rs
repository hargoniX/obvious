use core::fmt;

use crate::traits::Statement;

impl Statement for bool {
    #[inline(always)]
    fn evaluate(&self) -> bool {
        *self
    }
}

impl Statement for &bool {
    #[inline(always)]
    fn evaluate(&self) -> bool {
        **self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Or<L: Statement, R: Statement> {
    left: L,
    right: R,
}

impl<L: Statement, R: Statement> Or<L, R> {
    #[inline(always)]
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L: Statement, R: Statement> Statement for Or<L, R> {
    #[inline(always)]
    fn evaluate(&self) -> bool {
        self.left.evaluate() || self.right.evaluate()
    }
}

impl<L: Statement, R: Statement> fmt::Display for Or<L, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} \\lor {})", self.left, self.right)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct And<L: Statement, R: Statement> {
    left: L,
    right: R,
}

impl<L: Statement, R: Statement> And<L, R> {
    #[inline(always)]
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L: Statement, R: Statement> Statement for And<L, R> {
    #[inline(always)]
    fn evaluate(&self) -> bool {
        self.left.evaluate() && self.right.evaluate()
    }
}

impl<L: Statement, R: Statement> fmt::Display for And<L, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} \\wedge {})", self.left, self.right)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Not<T: Statement> {
    inner: T,
}

impl<T: Statement> Not<T> {
    #[inline(always)]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Statement> Statement for Not<T> {
    #[inline(always)]
    fn evaluate(&self) -> bool {
        !self.inner.evaluate()
    }
}

impl<T: Statement> fmt::Display for Not<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\overline{{{}}}", self.inner)
    }
}
