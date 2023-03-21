// mod extractors;
mod visitor;
use crate::{
    antlr::{yggdrasilantlrlexer::YggdrasilAntlrLexer, yggdrasilantlrparser::*, yggdrasilantlrvisitor::YggdrasilAntlrVisitor},
    traits::Extractor,
};
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    errors::ANTLRError,
    parser_rule_context::ParserRuleContext,
    tree::{ParseTree, ParseTreeVisitorCompat},
    InputStream,
};
use std::ops::Range;
use yggdrasil_ir::grammar::GrammarInfo;

#[derive(Clone, Debug, Default)]
pub struct YggdrasilANTLR {
    grammar: GrammarInfo,
    dirty: (),
}

impl YggdrasilANTLR {
    pub fn parse(input: &str) -> Result<GrammarInfo, ANTLRError> {
        let codepoints = input.chars().map(|x| x as u32).collect::<Vec<_>>();
        let input = InputStream::new(&*codepoints);
        let lexer = YggdrasilAntlrLexer::new(input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = YggdrasilAntlrParser::new(token_source);
        let root = parser.program()?;
        let mut state = YggdrasilANTLR::default();
        state.visit(&*root);
        Ok(state.grammar)
    }
}