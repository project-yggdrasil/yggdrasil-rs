use std::str::FromStr;

use yggdrasil_error::YggdrasilError;
use yggdrasil_parser::{
    bootstrap::{
        AtomicNode, BooleanNode, ClassStatementNode, ExpressionHardNode, ExpressionNode, ExpressionSoftNode, ExpressionTagNode,
        GrammarStatementNode, GroupPairNode, GroupStatementNode, IdentifierNode, PrefixNode, RootNode, StatementNode, StringNode, SuffixNode,
        TermNode, UnionBranchNode, UnionStatementNode,
    },
    TakeAnnotations, YggdrasilNode,
};

use crate::{
    data::{YggdrasilRegex, YggdrasilText},
    grammar::GrammarInfo,
    nodes::{UnaryExpression, YggdrasilExpression, YggdrasilOperator},
    rule::{GrammarAtomic, GrammarBody, GrammarRule, YggdrasilIdentifier},
};

mod annotations;

impl FromStr for GrammarInfo {
    type Err = YggdrasilError;

    fn from_str(s: &str) -> Result<Self, YggdrasilError> {
        GrammarInfo::try_from(RootNode::from_str(s)?)
    }
}

impl TryFrom<RootNode> for GrammarInfo {
    type Error = YggdrasilError;

    fn try_from(value: RootNode) -> Result<Self, Self::Error> {
        let mut out = GrammarInfo::default();
        for s in &value.statement {
            match s {
                StatementNode::GrammarStatement(v) => out.visit_grammar(v)?,
                StatementNode::ClassStatement(v) => match GrammarRule::build_class(v) {
                    Ok(o) => {
                        out.rules.insert(o.name.text.clone(), o);
                    }
                    Err(e) => {
                        println!("{e:?}");
                        println!("Class: {}", v.class_block.expression.text);
                    }
                },
                StatementNode::UnionStatement(v) => match GrammarRule::build_union(v) {
                    Ok(o) => {
                        out.rules.insert(o.name.text.clone(), o);
                    }
                    Err(e) => {
                        println!("{e:?}");
                        for i in &v.union_block.union_branch {
                            println!("Union: {}", i.expression.text);
                        }
                    }
                },
                StatementNode::GroupStatement(v) => match GrammarRule::build_group(v) {
                    Ok((id, terms)) => match id {
                        Some(id) => {
                            let mut name = vec![];
                            for o in terms {
                                name.push(o.name.clone());
                                out.rules.insert(o.name.text.clone(), o);
                            }
                            out.token_sets.insert(id.text.clone(), name);
                        }
                        None => {
                            for o in terms {
                                out.rules.insert(o.name.text.clone(), o);
                            }
                        }
                    },
                    Err(e) => {
                        println!("{e:?}");
                    }
                },
            }
        }
        Ok(out)
    }
}

impl GrammarInfo {
    fn visit_grammar(&mut self, node: &GrammarStatementNode) -> Result<(), YggdrasilError> {
        self.name = YggdrasilIdentifier::build(&node.identifier);
        Ok(())
    }
}

impl GrammarRule {
    fn build_class(node: &ClassStatementNode) -> Result<Self, YggdrasilError> {
        let name = YggdrasilIdentifier::build(&node.name);
        let rule = Self {
            name,
            body: GrammarBody::Class { term: YggdrasilExpression::build_or(&node.class_block.expression)? },
            range: node.get_range().unwrap_or_default(),
            ..Default::default()
        }
        .with_annotation(node.annotations());
        Ok(rule)
    }
    fn build_class_in_group(node: &GroupPairNode) -> Result<Self, YggdrasilError> {
        let name = YggdrasilIdentifier::build(&node.identifier);
        let rule = Self {
            name,
            body: GrammarBody::Class { term: YggdrasilExpression::build_atomic(&node.atomic)? },
            range: node.get_range().unwrap_or_default(),
            ..Default::default()
        };
        Ok(rule)
    }
    fn build_union(node: &UnionStatementNode) -> Result<Self, YggdrasilError> {
        let name = YggdrasilIdentifier::build(&node.name);
        let mut branches = vec![];
        for branch in &node.union_block.union_branch {
            match YggdrasilExpression::build_tag_branch(branch) {
                Ok(o) => branches.push(o),
                Err(_) => {}
            }
        }
        let rule = Self { name, body: GrammarBody::Union { branches }, range: node.get_range().unwrap_or_default(), ..Default::default() };
        Ok(rule)
    }
    fn build_group(node: &GroupStatementNode) -> Result<(Option<YggdrasilIdentifier>, Vec<Self>), YggdrasilError> {
        let name = node.identifier.as_ref().map(YggdrasilIdentifier::build);
        let mut out = vec![];
        for term in &node.group_block.group_pair {
            match GrammarRule::build_class_in_group(term) {
                Ok(o) => out.push(o.with_annotation(node.annotations())),
                Err(_) => {}
            }
        }
        Ok((name, out))
    }
}

impl YggdrasilExpression {
    fn build_or(node: &ExpressionNode) -> Result<Self, YggdrasilError> {
        match node.expression_hard.as_slice() {
            [head, rest @ ..] => {
                let mut head = YggdrasilExpression::build_hard(head)?;
                for term in rest {
                    head |= YggdrasilExpression::build_hard(term)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class or", node.get_range().unwrap_or_default()))?,
        }
    }

    fn build_hard(node: &ExpressionHardNode) -> Result<Self, YggdrasilError> {
        match node.expression_soft.as_slice() {
            [head, rest @ ..] => {
                let mut head = YggdrasilExpression::build_soft(head)?;
                for term in rest {
                    head += YggdrasilExpression::build_soft(term)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class hard", node.get_range().unwrap_or_default()))?,
        }
    }
    fn build_soft(node: &ExpressionSoftNode) -> Result<Self, YggdrasilError> {
        match node.expression_tag.as_slice() {
            [head, rest @ ..] => {
                let mut head = YggdrasilExpression::build_tag_node(head)?;
                for term in rest {
                    head &= YggdrasilExpression::build_tag_node(term)?;
                }
                Ok(head)
            }
            _ => Err(YggdrasilError::syntax_error("empty class soft", node.get_range().unwrap_or_default()))?,
        }
    }
    fn build_tag_branch(node: &UnionBranchNode) -> Result<(Option<YggdrasilIdentifier>, Self), YggdrasilError> {
        let id = node.branch_tag.as_ref().map(|o| YggdrasilIdentifier::build(&o.identifier));
        let expr = YggdrasilExpression::build_hard(&node.expression)?;
        Ok((id, expr))
    }
    fn build_tag_node(node: &ExpressionTagNode) -> Result<Self, YggdrasilError> {
        match node.term.as_slice() {
            [last] => {
                let expr = YggdrasilExpression::build_term(last)?;
                Ok(expr)
            }
            [first, last] => {
                let id = YggdrasilExpression::build_term(first)?;
                let mut expr = YggdrasilExpression::build_term(last)?;
                expr.tag = id.as_identifier().cloned();
                Ok(expr)
            }
            _ => Err(YggdrasilError::syntax_error("FIXME: TAG MODE", node.get_range().unwrap_or_default()))?,
        }
    }
    fn build_term(node: &TermNode) -> Result<Self, YggdrasilError> {
        let mut base = YggdrasilExpression::build_atomic(&node.atomic)?;
        let mut unary = Vec::with_capacity(node.prefix.len() + node.suffix.len());
        for i in &node.suffix {
            match i {
                SuffixNode::Suffix0 => unary.push(YggdrasilOperator::OPTIONAL),
                SuffixNode::Suffix1 => unary.push(YggdrasilOperator::REPEATS),
                SuffixNode::Suffix2 => unary.push(YggdrasilOperator::REPEAT1),
            }
        }
        for i in node.prefix.iter().rev() {
            match i {
                PrefixNode::Prefix0 => unary.push(YggdrasilOperator::Negative),
                PrefixNode::Prefix1 => unary.push(YggdrasilOperator::Positive),
                PrefixNode::Prefix2 => base.remark = true,
            }
        }
        if unary.is_empty() { Ok(base) } else { Ok(UnaryExpression { base: Box::new(base), operators: unary }.into()) }
    }
    fn build_atomic(node: &AtomicNode) -> Result<Self, YggdrasilError> {
        let expr = match node {
            AtomicNode::Atomic0(e) => YggdrasilExpression::build_or(e)?,
            AtomicNode::Boolean(v) => match v {
                BooleanNode::Boolean0 => YggdrasilExpression::boolean(true),
                BooleanNode::Boolean1 => YggdrasilExpression::boolean(true),
            },
            AtomicNode::FunctionCall(v) => {
                todo!()
            }
            AtomicNode::Identifier(v) => YggdrasilIdentifier::build(v).into(),
            AtomicNode::RegexEmbed(v) => YggdrasilRegex::new(v.text.trim_matches('/'), v.get_range().unwrap_or_default()).into(),
            AtomicNode::RegexRange(v) => YggdrasilRegex::new(&v.text, v.get_range().unwrap_or_default()).into(),
            AtomicNode::String(v) => match v {
                StringNode::String0(s) => YggdrasilText::new(s.trim_matches('\''), Default::default()).into(),
                StringNode::String1(s) => YggdrasilText::new(s.trim_matches('"'), Default::default()).into(),
            },
        };
        Ok(expr)
    }
}

impl YggdrasilIdentifier {
    fn build(node: &IdentifierNode) -> Self {
        Self { text: node.text.clone(), range: node.get_range().unwrap_or_default() }
    }
}
