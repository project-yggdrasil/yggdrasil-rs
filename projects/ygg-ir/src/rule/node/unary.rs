use super::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub base: ExpressionNode,
    pub ops: Vec<Operator>,
}

impl From<UnaryExpression> for ExpressionKind {
    fn from(e: UnaryExpression) -> Self {
        Self::Unary(Box::new(e))
    }
}

impl Add<Operator> for ExpressionNode {
    type Output = Self;

    fn add(self, o: Operator) -> Self::Output {
        match self.kind {
            ExpressionKind::Unary(node) => {
                let mut ops = node.ops;
                ops.push(o);
                let unary = UnaryExpression { base: node.base, ops };
                ExpressionNode { kind: ExpressionKind::Unary(Box::new(unary)), tag: self.tag }
            }
            _ => {
                let unary = UnaryExpression { base: self, ops: vec![o] };
                ExpressionNode { kind: ExpressionKind::Unary(Box::new(unary)), tag: "".to_string() }
            }
        }
    }
}

impl Operator {
    pub fn prefix(o: &str) -> Operator {
        match o {
            "*" => Self::Recursive,
            "^" => Self::Remark,
            _ => unreachable!(),
        }
    }
    pub fn suffix(o: &str) -> Operator {
        match o {
            "?" => Self::Optional,
            "+" => Self::Repeats,
            "*" => Self::Repeat1,
            _ => unreachable!(),
        }
    }
}
