// use tree_sitter_cli::generate::{
//     generate_parser_for_grammar_with_opts, generate_parser_in_directory,
//     grammars::{InputGrammar, PrecedenceEntry, Variable, VariableType},
//     prepare_grammar::prepare_grammar,
//     rules::{Alias, Associativity, MetadataParams, Rule, Symbol, SymbolType},
//     GeneratedParser,
// };

use super::*;

mod expr;

use tree_sitter_cli::generate::{
    generate_parser_for_input_grammar,
    grammars::{InputGrammar, Variable, VariableType, VariableType::Named},
    parse_grammar::{parse_grammar, GrammarJSON},
    rules::{
        MetadataParams, Precedence, Rule,
        Rule::{Blank, Choice, Metadata, NamedSymbol, Repeat},
    },
};

impl GrammarState {
    pub fn build_input_grammar(&self) -> InputGrammar {
        InputGrammar {
            name: self.name.data.to_owned(),
            variables: self.variables(),
            extra_symbols: self.extra_symbols(),
            expected_conflicts: vec![],
            precedence_orderings: vec![],
            external_tokens: vec![],
            variables_to_inline: vec![],
            supertype_symbols: vec![],
            word_token: Some(String::from("id")),
        }
    }
    fn variables(&self) -> Vec<Variable> {
        self.named_rules().into_iter().map(|e| Variable::from(e)).collect()
    }

    fn extra_symbols(&self) -> Vec<Rule> {
        self.ignores.iter().cloned().map(|e| Rule::NamedSymbol(e.data)).collect()
    }
}