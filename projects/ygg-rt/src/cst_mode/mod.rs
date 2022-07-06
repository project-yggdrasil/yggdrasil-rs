use crate::{AstNode, NodeType};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{
    fmt::{Debug, Formatter},
    marker::PhantomData,
    ops::{Deref, Range},
};

pub mod context;
mod display;

pub type NodeID = u32;

/// The basic unit of semantic analysis.
///
/// A weakly typed node, which can be equivalent to:
///
/// ```
/// pub struct CSTNode<'i, R, M> {
///     meta: M,
///     kind: R,
///     slice: &'i str,
///     children: Vec<CSTNode<'i, R, M>>,
/// }
/// ```
/// Erase theses type to make it more flexible.
///
/// **This node is immutable**
///
/// If a modification occurs, a new clone must be generated.
#[derive(Clone, Debug)]
pub struct ConcreteNode {
    /// The kind of the node
    pub(crate) kind: i16,
    /// The offset in raw bytes, life time erased
    pub(crate) range: Range<u32>,
    pub(crate) children: Vec<ConcreteNode>,
}

impl ConcreteNode {
    /// Get the kind of the node
    pub fn get_kind<N>(&self) -> N
    where
        N: NodeType,
    {
        <N as From<i16>>::from(self.kind)
    }
    /// Set the kind of the node
    pub fn set_kind<N>(&mut self, kind: N)
    where
        N: NodeType,
    {
        self.kind = <N as Into<i16>>::into(kind);
    }
    /// Set the kind of the node
    pub fn with_kind<N>(mut self, kind: N) -> Self
    where
        N: NodeType,
    {
        self.set_kind(kind);
        self
    }
    /// Get the children of the node
    pub fn get_children(&self) -> &[ConcreteNode] {
        &self.children
    }
    /// Add a child to the node
    pub fn add_child(&mut self, child: ConcreteNode) {
        self.children.push(child);
    }
    /// Add a child to the node
    pub fn set_children(&mut self, children: Vec<ConcreteNode>) {
        self.children = children;
    }
    /// Add a child to the node
    pub fn with_children(mut self, children: Vec<ConcreteNode>) -> Self {
        self.set_children(children);
        self
    }
    /// Get the range of the node
    pub fn get_range(&self) -> Range<usize> {
        Range { start: self.range.start as usize, end: self.range.end as usize }
    }
    /// Set the range of the node
    pub fn set_range(&mut self, start: usize, end: usize) {
        self.range = Range { start: start as u32, end: end as u32 };
    }
    /// Set the range of the node
    pub fn with_range(mut self, start: usize, end: usize) -> Self {
        self.set_range(start, end);
        self
    }
}

impl ConcreteNode {
    /// Check if the node is one of the given types
    ///
    /// # Arguments
    ///
    /// * `kind`:
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    /// # use yggdrasil_rt::ConcreteNode;
    /// enum JsonNode {
    ///     Object,
    ///     Array,
    /// }
    /// let node = ConcreteNode::new(0).with_kind(JsonNode::Object);
    /// assert!(node.is_a(&[JsonNode::Object]));
    /// ```
    pub fn is_a<N>(&self, kind: &[N]) -> bool
    where
        N: NodeType,
    {
        for node in kind {
            if self.kind == <N as Into<i16>>::into(*node) {
                return true;
            }
        }
        return false;
    }
}
