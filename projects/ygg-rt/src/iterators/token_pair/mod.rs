use alloc::{format, rc::Rc, vec::Vec};
use core::{
    borrow::Borrow,
    fmt,
    hash::{Hash, Hasher},
    ptr, str,
};

use super::{
    token_queue::TokenQueue,
    token_tree::TokenTree,
    tokens::{self, Tokens},
};
use crate::{span::TextSpan, YggdrasilRule};

/// A matching pair of [`Token`]s and everything between them.
///
/// A matching `Token` pair is formed by a `Token::Start` and a subsequent `Token::End` with the
/// same `Rule`, with the condition that all `Token`s between them can form such pairs as well.
/// This is similar to the [brace matching problem](https://en.wikipedia.org/wiki/Brace_matching) in
/// editors.
///
/// [`Token`]: ../enum.Token.html
#[derive(Clone)]
pub struct TokenPair<'i, R> {
    /// # Safety
    ///
    /// All `QueueableToken`s' `input_pos` must be valid character boundary indices into `input`.
    queue: Rc<Vec<TokenQueue<R>>>,
    input: &'i str,
    /// Token index into `queue`.
    start: usize,
}

/// # Safety
///
/// All `QueueableToken`s' `input_pos` must be valid character boundary indices into `input`.
pub unsafe fn new<R: YggdrasilRule>(queue: Rc<Vec<TokenQueue<R>>>, input: &str, start: usize) -> TokenPair<R> {
    TokenPair { queue, input, start }
}

impl<'i, R: YggdrasilRule> TokenPair<'i, R> {
    /// Returns the `Rule` of the `Pair`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    /// }
    ///
    /// let input = "";
    /// let pair = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// })
    /// .unwrap()
    /// .next()
    /// .unwrap();
    ///
    /// assert_eq!(pair.as_rule(), Rule::a);
    /// ```
    #[inline]
    pub fn as_rule(&self) -> R {
        match &self.queue[self.pair()] {
            TokenQueue::End { rule, .. } => rule.clone(),
            _ => unreachable!(),
        }
    }

    /// Captures a slice from the `&str` defined by the token `Pair`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab,
    /// }
    ///
    /// let input = "ab";
    /// let pair = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pair with Rule::ab ...
    /// #     state.rule(Rule::ab, |s| s.match_string("ab"))
    /// })
    /// .unwrap()
    /// .next()
    /// .unwrap();
    ///
    /// assert_eq!(pair.as_str(), "ab");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'i str {
        let start = self.pos(self.start);
        let end = self.pos(self.pair());

        // Generated positions always come from Positions and are UTF-8 borders.
        &self.input[start..end]
    }

    /// Returns the input string of the `Pair`.
    ///
    /// This function returns the input string of the `Pair` as a `&str`. This is the source string
    /// from which the `Pair` was created. The returned `&str` can be used to examine the contents of
    /// the `Pair` or to perform further processing on the string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab,
    /// }
    ///
    /// // Example: Get input string from a Pair
    ///
    /// let input = "ab";
    /// let pair = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pair with Rule::ab ...
    /// #     state.rule(Rule::ab, |s| s.match_string("ab"))
    /// })
    /// .unwrap()
    /// .next()
    /// .unwrap();
    ///
    /// assert_eq!(pair.as_str(), "ab");
    /// assert_eq!(input, pair.get_input());
    /// ```
    pub fn get_input(&self) -> &'i str {
        self.input
    }

    /// Returns the `Span` defined by the `Pair`, **without** consuming it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab,
    /// }
    ///
    /// let input = "ab";
    /// let pair = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pair with Rule::ab ...
    /// #     state.rule(Rule::ab, |s| s.match_string("ab"))
    /// })
    /// .unwrap()
    /// .next()
    /// .unwrap();
    ///
    /// assert_eq!(pair.as_span().as_str(), "ab");
    /// ```
    #[inline]
    pub fn as_span(&self) -> TextSpan<'i> {
        let start = self.pos(self.start);
        let end = self.pos(self.pair());

        // Generated positions always come from Positions and are UTF-8 borders.
        unsafe { TextSpan::new_unchecked(self.input, start, end) }
    }

    /// Get current node tag
    #[inline]
    pub fn as_node_tag(&self) -> Option<&str> {
        match &self.queue[self.pair()] {
            TokenQueue::End { tag, .. } => tag.as_ref().map(|x| x.borrow()),
            _ => None,
        }
    }

    /// Returns the inner `Pairs` between the `Pair`, consuming it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    /// }
    ///
    /// let input = "";
    /// let pair = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// })
    /// .unwrap()
    /// .next()
    /// .unwrap();
    ///
    /// assert!(pair.into_inner().next().is_none());
    /// ```
    #[inline]
    pub fn into_inner(self) -> TokenTree<'i, R> {
        let pair = self.pair();
        TokenTree::new(self.queue, self.input, self.start + 1, pair)
    }
    /// Returns the inner `Pairs` between the `Pair`, consuming it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    /// }
    ///
    /// let input = "";
    /// let pair = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// })
    /// .unwrap()
    /// .next()
    /// .unwrap();
    ///
    /// assert!(pair.into_inner().next().is_none());
    /// ```
    pub fn has_child(&self) -> bool {
        self.clone().into_inner().len() != 0
    }

    /// Returns the `Tokens` for the `Pair`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use yggdrasil_rt;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    /// }
    ///
    /// let input = "";
    /// let pair = yggdrasil_rt::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// })
    /// .unwrap()
    /// .next()
    /// .unwrap();
    /// let tokens: Vec<_> = pair.tokens().collect();
    ///
    /// assert_eq!(tokens.len(), 2);
    /// ```
    #[inline]
    pub fn tokens(self) -> Tokens<'i, R> {
        let end = self.pair();

        tokens::new(self.queue, self.input, self.start, end + 1)
    }

    fn pair(&self) -> usize {
        match self.queue[self.start] {
            TokenQueue::Start { end_token_index, .. } => end_token_index,
            _ => unreachable!(),
        }
    }

    fn pos(&self, index: usize) -> usize {
        match self.queue[index] {
            TokenQueue::Start { input_offset: input_pos, .. } | TokenQueue::End { input_offset: input_pos, .. } => input_pos,
        }
    }
}

impl<'i, R: YggdrasilRule> TokenTree<'i, R> {
    /// Create a new `Pairs` iterator containing just the single `Pair`.
    pub fn single(pair: TokenPair<'i, R>) -> Self {
        let end = pair.pair();
        TokenTree::new(pair.queue, pair.input, pair.start, end)
    }
}

impl<'i, R: YggdrasilRule> fmt::Debug for TokenPair<'i, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pair = &mut f.debug_struct("Pair");
        pair.field("rule", &self.as_rule());
        // In order not to break compatibility
        if let Some(s) = self.as_node_tag() {
            pair.field("node_tag", &s);
        }
        pair.field("span", &self.as_span()).field("inner", &self.clone().into_inner().collect::<Vec<_>>()).finish()
    }
}

impl<'i, R: YggdrasilRule> fmt::Display for TokenPair<'i, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rule = self.as_rule();
        let start = self.pos(self.start);
        let end = self.pos(self.pair());
        let mut pairs = self.clone().into_inner().peekable();

        if pairs.peek().is_none() {
            write!(f, "{:?}({}, {})", rule, start, end)
        }
        else {
            write!(
                f,
                "{:?}({}, {}, [{}])",
                rule,
                start,
                end,
                pairs.map(|pair| format!("{}", pair)).collect::<Vec<_>>().join(", ")
            )
        }
    }
}

impl<'i, R: PartialEq> PartialEq for TokenPair<'i, R> {
    fn eq(&self, other: &TokenPair<'i, R>) -> bool {
        Rc::ptr_eq(&self.queue, &other.queue) && ptr::eq(self.input, other.input) && self.start == other.start
    }
}

impl<'i, R: Eq> Eq for TokenPair<'i, R> {}

impl<'i, R: Hash> Hash for TokenPair<'i, R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&*self.queue as *const Vec<TokenQueue<R>>).hash(state);
        (self.input as *const str).hash(state);
        self.start.hash(state);
    }
}