use crate::errors::ObviousError;

use core::fmt;

use crate::conclusion::{Equivalence, Implication};
use crate::logic::{And, Not, Or};
use crate::variable::Variable;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Statements {
    And(And),
    Or(Or),
    Not(Not),
    Implication(Implication),
    Equivalence(Equivalence),
    Boolean(bool),
    Variable(Variable),
}

impl fmt::Display for Statements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::And(i) => write!(f, "{}", i),
            Self::Or(i) => write!(f, "{}", i),
            Self::Not(i) => write!(f, "{}", i),
            Self::Implication(i) => write!(f, "{}", i),
            Self::Equivalence(i) => write!(f, "{}", i),
            Self::Boolean(i) => write!(f, "{}", i),
            Self::Variable(i) => write!(f, "{}", i),
        }
    }
}

impl Statements {
    #[inline(always)]
    pub fn or(&self, rhs: &Statements) -> Statements {
        Self::Or(Or::new(self.clone(), rhs.clone()))
    }

    #[inline(always)]
    pub fn and(&self, rhs: &Statements) -> Statements {
        Self::And(And::new(self.clone(), rhs.clone()))
    }

    #[inline(always)]
    pub fn not(&self) -> Statements {
        Self::Not(Not::new(self.clone()))
    }

    #[inline(always)]
    pub fn implies(&self, rhs: &Statements) -> Statements {
        Self::Implication(Implication::new(self.clone(), rhs.clone()))
    }

    #[inline(always)]
    pub fn equates(&self, rhs: &Statements) -> Statements {
        Self::Equivalence(Equivalence::new(self.clone(), rhs.clone()))
    }

    #[inline(always)]
    pub fn assume_true(&self) -> Statements {
        Self::Boolean(true)
    }

    #[inline(always)]
    pub fn assume_false(&self) -> Statements {
        Self::Boolean(false)
    }
}

impl Evaluatable for Statements {
    fn evaluate(&self) -> Result<bool, ObviousError> {
        match self {
            Self::And(i) => i.evaluate(),
            Self::Or(i) => i.evaluate(),
            Self::Not(i) => i.evaluate(),
            Self::Implication(i) => i.evaluate(),
            Self::Equivalence(i) => i.evaluate(),
            Self::Boolean(i) => i.evaluate(),
            Self::Variable(i) => i.evaluate(),
        }
    }

    fn evaluate_with_variables(
        &self,
        variables: &HashMap<String, bool>,
    ) -> Result<bool, ObviousError> {
        match self {
            Self::And(i) => i.evaluate_with_variables(variables),
            Self::Or(i) => i.evaluate_with_variables(variables),
            Self::Not(i) => i.evaluate_with_variables(variables),
            Self::Implication(i) => i.evaluate_with_variables(variables),
            Self::Equivalence(i) => i.evaluate_with_variables(variables),
            Self::Boolean(i) => i.evaluate_with_variables(variables),
            Self::Variable(i) => i.evaluate_with_variables(variables),
        }
    }
}

pub trait Evaluatable: Clone + fmt::Display {
    fn evaluate(&self) -> Result<bool, ObviousError>;
    fn evaluate_with_variables(
        &self,
        variables: &HashMap<String, bool>,
    ) -> Result<bool, ObviousError>;
}
