use crate::conclusion::*;
use crate::logic::*;
use core::fmt;

pub trait Statement: Clone + Copy + fmt::Display {
    fn evaluate(&self) -> bool;

    #[inline(always)]
    fn or<T>(self, rhs: T) -> Or<Self, T>
    where
        T: Statement,
    {
        Or::new(self, rhs)
    }

    #[inline(always)]
    fn and<T>(self, rhs: T) -> And<Self, T>
    where
        T: Statement,
    {
        And::new(self, rhs)
    }

    #[inline(always)]
    fn not(self) -> Not<Self> {
        Not::new(self)
    }

    #[inline(always)]
    fn implies<T>(self, rhs: T) -> Implication<Self, T>
    where
        T: Statement,
    {
        Implication::new(self, rhs)
    }

    #[inline(always)]
    fn equates<T>(self, rhs: T) -> Equivalence<Self, T>
    where
        T: Statement,
    {
        Equivalence::new(self, rhs)
    }

    #[inline(always)]
    fn assume_true(self) -> bool {
        true
    }

    #[inline(always)]
    fn assume_false(self) -> bool {
        false
    }
}
