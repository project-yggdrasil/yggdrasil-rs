#![feature(once_cell)]

mod codegen;
mod debug;
#[allow(unused)]
mod ygg;

pub use tree_sitter::{Parser, Tree};

pub use tree_sitter_yg::language;

pub use self::ygg::SyntaxKind;
