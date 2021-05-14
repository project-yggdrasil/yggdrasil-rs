use super::*;

impl Debug for YGGRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("name", &self.name.data)
            .field("type", &self.ty.data)
            .field("force_inline", &self.force_inline)
            .field("already_inline", &self.already_inline)
            .field("eliminate_unmarked", &self.eliminate_unmarked)
            .field("eliminate_unnamed", &self.eliminate_unnamed)
            .field("expression", &self.expression)
            .field("range", &self.range)
            .finish()
    }
}

impl Debug for ExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("ExpressionNode");
        if let Some(s) = &self.tag {
            w.field("tag", &s.tag.data);
            w.field("tag_mode", &s.mode);
        }
        if let Some(s) = &self.field {
            w.field("field", &s.data);
        }
        if let Some(s) = &self.ty {
            w.field("type", &s.data);
        }
        if self.inline_token {
            w.field("inline_token", &true);
        }
        w.field("base", &self.node);
        w.finish()
    }
}

impl Debug for RefinedExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Data(e) => {e.fmt(f)}
            Self::Unary(e) => {e.fmt(f)}
            Self::Choice(e) => {e.fmt(f)}
            Self::Concat(e) => {e.fmt(f)}
        }
    }
}

impl Debug for RefinedChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Choice")?;
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl Debug for RefinedConcat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Concat")?;
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl Debug for RefinedUnary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Unary")
            .field("base", &self.base)
            .field("operations", &self.ops)
            .finish()
    }
}

impl Debug for RefinedData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(e) => {e.fmt(f)}
            Self::Regex(e) => {e.fmt(f)}
            Self::String(e) => {f.write_str(e)}
            Self::Integer(e) => {f.write_str(&e.to_string())}
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Optional => {f.write_str("Optional?")}
            Self::Repeats => {f.write_str("Repeats*")}
            Self::Repeats1 => {f.write_str("RepeatsNonnull+")}
            Self::Mark => {f.write_str("^MarkSymbol")}
            Self::Recursive => {f.write_str("*RecursiveSymbol")}
        }
    }
}