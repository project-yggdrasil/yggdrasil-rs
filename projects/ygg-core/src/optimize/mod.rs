use itertools::Itertools;
use std::{collections::HashSet, mem::take};
use yggdrasil_error::YggdrasilError;
use yggdrasil_ir::{
    grammar::GrammarInfo,
    nodes::{ChoiceExpression, ConcatExpression, ExpressionKind, UnaryExpression, YggdrasilExpression},
    rule::{FunctionRule, GrammarAtomic, GrammarRule},
    traits::{CodeOptimizer, FieldDescriptor},
    IndexMap,
};

mod dead_code;
mod emit_function;
mod fuse_rule;
mod inlining;
mod insert_ignore;
mod remark_tag;

mod refine;

pub use self::{
    dead_code::DeadCodeEliminator, emit_function::EmitFunction, fuse_rule::FusionRules, inlining::InlineRules,
    insert_ignore::InsertIgnore, refine::RefineRules, remark_tag::RemarkTags,
};
