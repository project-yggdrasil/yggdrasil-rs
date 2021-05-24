use super::*;

impl YGGMarker {
    pub fn split_whitespace<'i>(&self, nodes: &mut Vec<CSTNode<'i>>, pairs: Pair<'i,Rule>) ->  RuleResult<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::COMMENT|Rule::WHITE_SPACE|Rule::NEWLINE => {nodes.push(self.atomic(pair, None)?)}
                _ => unreachable!()
            }
        }
        Ok(())
    }

    pub fn atomic<'i>(&self, pairs: Pair<'i, Rule>, mark: Option<&'static str>) ->  RuleResult<CSTNode<'i>>  {
        let position = get_position(&pairs);
        let text = pairs.as_str();
        Ok(CSTNode {
            text,
            mark,
            position,
            children: vec![],
        })
    }
}
