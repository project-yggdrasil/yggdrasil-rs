use std::{collections::BTreeMap, ops::Range};

use crate::frontend::rule::Expression;
use indexmap::map::IndexMap;
use lsp_types::Url;

// mod optimize;
pub mod rule;
// pub mod typing;

#[derive(Clone, Debug)]
pub struct GrammarInfo {
    /// File path of the grammar
    pub url: Option<Url>,
    pub text: String,
    pub is_grammar: bool,
    pub name: Symbol,
    pub extensions: Vec<Symbol>,
    pub ignores: Vec<Symbol>,
    pub imports: BTreeMap<Url, Vec<SymbolAlias>>,
    pub rules: BTreeMap<String, GrammarRule>,
    pub rule_prefix: String,
    pub rule_suffix: String,
}

pub struct Symbol {
    pub name: String,
    pub range: Range<usize>,
}

pub struct SymbolAlias {
    pub name: String,
    pub alias: String,
    pub range: Range<usize>,
}

#[derive(Clone)]
pub struct GrammarRule {
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// def RuleName {
    ///
    /// }
    /// ```
    pub name: String,
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// def rule -> char {
    ///
    /// }
    ///
    /// def rule() -> char {
    ///
    /// }
    /// ```
    pub r#type: String,
    /// Document of this rule
    ///
    /// ## Examples
    /// ```ygg
    /// 
    /// def rule {
    ///
    /// }
    ///
    /// def rule() -> char {
    ///
    /// }
    /// ```
    pub document: String,
    ///
    pub derives: RuleDerive,
    /// Automatically inline when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// #inline(true)
    /// def rule {
    ///
    /// }
    ///
    /// def inline rule {
    ///
    /// }
    ///
    /// def _rule {
    ///
    /// }
    /// ```
    pub auto_inline: bool,
    /// Automatically box when this rule is called
    ///
    /// ## Examples
    /// ```ygg
    /// #boxed(true)
    /// def rule {
    ///
    /// }
    ///
    /// def boxed rule {
    ///
    /// }
    /// ```
    pub auto_boxed: bool,
    pub auto_capture: bool,
    pub atomic_rule: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// name <- expr
    /// ^expr
    /// ```
    pub eliminate_unmarked: bool,
    /// Eliminate unnamed nodes
    /// ```ygg
    /// "string"
    /// /regex/
    /// [0-9a-z]
    /// 012345
    /// ```
    pub eliminate_unnamed: bool,
    ///
    pub body: Expression,
    /// position of all parts
    pub range: Range<usize>,
}

#[derive(Clone)]
pub struct RuleDerive {
    pub(crate) parser: Option<String>,
    pub(crate) debug: Option<String>,
    pub(crate) display: Option<String>,
    pub(crate) eq: bool,
    pub(crate) eq_partial: Option<String>,
    pub(crate) ord: bool,
    pub(crate) ord_partial: Option<String>,
    pub(crate) hash: Option<String>,
}

impl Default for RuleDerive {
    fn default() -> Self {
        Self {
            parser: None,
            debug: None,
            display: None,
            eq: false,
            eq_partial: None,
            ord: false,
            ord_partial: None,
            hash: None,
        }
    }
}
