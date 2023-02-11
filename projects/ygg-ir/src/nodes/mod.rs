pub use self::{choice::ChoiceExpression, concat::ConcatExpression, unary::UnaryExpression};
use crate::{
    data::{DataKind, RuleReference},
    rule::{GrammarRule, GrammarRuleKind},
    FunctionExpression,
};
use convert_case::{Case, Casing};
use diagnostic_quick::{QError, QResult};
use indexmap::{IndexMap, IndexSet};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    mem::take,
    ops::{Add, BitAnd, BitOr, BitXor},
    slice::{Iter, IterMut},
};

mod choice;
mod concat;
mod debug;
mod expr;
mod unary;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ExpressionNode {
    pub kind: ExpressionKind,
    pub tag: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ExpressionKind {
    Function(Box<FunctionExpression>),
    Choice(Box<ChoiceExpression>),
    Concat(Box<ConcatExpression>),
    Unary(Box<UnaryExpression>),
    Rule(Box<RuleReference>),
    Data(Box<DataKind>),
}

impl From<RuleReference> for ExpressionNode {
    fn from(value: RuleReference) -> Self {
        Self { kind: ExpressionKind::Rule(Box::new(value)), tag: "".to_string() }
    }
}

impl ExpressionNode {
    pub fn unary(mut base: ExpressionNode, o: Operator) -> Self {
        match base.kind {
            ExpressionKind::Unary(ref mut v) if base.tag.is_empty() => {
                v.ops.push(o);
                base
            }
            _ => Self { kind: ExpressionKind::Unary(Box::new(UnaryExpression { base, ops: vec![o] })), tag: "".to_string() },
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Operator {
    /// ```ygg
    /// !e
    /// ```
    Negative,
    /// e?
    Optional,
    /// e*
    Repeats,
    /// e+
    Repeat1,
    /// no such literal
    Boxing,
    /// e+
    RepeatsBetween(Option<u8>, Option<u8>),
    /// ^e
    Remark,
    /// *e
    Recursive,
}
