mod constants;
mod dynamic;
mod escapes;
mod keywords;
mod macros;

use self::{
    constants::COMPLETE_CONSTANTS, escapes::COMPLETE_ESCAPES, keywords::COMPLETE_KEYWORDS,
    macros::COMPLETE_MACROS,
};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, lazy::SyncLazy};
use tower_lsp::lsp_types::{
    CompletionItem,
    CompletionItemKind::{self, *},
    CompletionOptions, CompletionParams, CompletionResponse, CompletionTriggerKind, Documentation,
    InsertTextFormat, MarkupContent, MarkupKind, WorkDoneProgressOptions,
};

pub static COMPLETION_OPTIONS: SyncLazy<CompletionOptions> = SyncLazy::new(|| {
    let completion_trigger = vec!['@', '\\', '`'];
    CompletionOptions {
        resolve_provider: Some(false),
        trigger_characters: Some(completion_trigger.iter().map(ToString::to_string).collect()),
        all_commit_characters: None,
        work_done_progress_options: WorkDoneProgressOptions { work_done_progress: Some(false) },
        completion_item: None,
    }
});

pub async fn completion_provider(p: CompletionParams) -> Option<CompletionResponse> {
    let ctx = match p.context {
        Some(s) => s,
        None => return None,
    };
    match ctx.trigger_kind {
        CompletionTriggerKind::Invoked => {
            let mut items = vec![];
            items.extend(COMPLETE_CONSTANTS.to_owned());
            if p.text_document_position.position.character <= 1 {
                items.extend(COMPLETE_KEYWORDS.to_owned());
            }
            // TODO: add dynamic symbols
            Some(CompletionResponse::Array(items))
        }
        CompletionTriggerKind::TriggerCharacter => {
            match ctx.trigger_character.and_then(|e| e.chars().next()) {
                Some('@') => Some(CompletionResponse::Array(COMPLETE_MACROS.to_owned())),
                Some('\\') => Some(CompletionResponse::Array(COMPLETE_ESCAPES.to_owned())),
                Some('`') => Some(CompletionResponse::Array(list_completion_kinds())),
                _ => None,
            }
        }
        CompletionTriggerKind::TriggerForIncompleteCompletions => {
            // unimplemented!()
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentString {
    cmd: String,
    short: String,
    long: String,
}

impl DocumentString {
    pub fn new(cmd: &str, short: &str, long: &str) -> DocumentString {
        Self {
            cmd: String::from(cmd.trim()),
            short: String::from(short.trim()),
            long: String::from(long.trim()),
        }
    }
}

fn load_md_doc(input: &str) -> Vec<DocumentString> {
    let mut out = VecDeque::new();
    let mut cmd = "";
    let mut short = "";
    let mut long = String::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("# ") {
            out.push_back(DocumentString::new(cmd, short, &long));
            cmd = &line[2..line.len()];
            short = lines.next().unwrap();
            long = String::new()
        }
        else {
            long.push_str(line);
            long.push('\n');
        }
    }
    out.push_back(DocumentString::new(cmd, short, &long));
    out.pop_front();
    return Vec::from(out);
}

#[allow(dead_code)]
fn list_completion_kinds() -> Vec<CompletionItem> {
    fn item(e: CompletionItemKind) -> CompletionItem {
        CompletionItem { label: format!("{:?}", e), kind: Some(e), ..CompletionItem::default() }
    }
    vec![
        item(Text),
        item(Method),
        item(Function),
        item(Constructor),
        item(Field),
        item(Variable),
        item(Class),
        item(Interface),
        item(Module),
        item(Property),
        item(Unit),
        item(Value),
        item(Enum),
        item(Keyword),
        item(Snippet),
        item(Color),
        item(File),
        item(Reference),
        item(Folder),
        item(EnumMember),
        item(Constant),
        item(Struct),
        item(Event),
        item(Operator),
        item(TypeParameter),
    ]
}

#[test]
fn check_yaml() {
    println!("{:#?}", load_md_doc(include_str!("escapes/escapes.md")));
    println!("{:#?}", load_md_doc(include_str!("escapes/escapes_args.md")));
    println!("{:#?}", load_md_doc(include_str!("macros/macros.md")));
    println!("{:#?}", load_md_doc(include_str!("macros/macros_args.md")));
    println!("{:#?}", load_md_doc(include_str!("constants/constants_ascii.md")));
    println!("{:#?}", load_md_doc(include_str!("constants/constants_op.md")));
    println!("{:#?}", load_md_doc(include_str!("constants/constants_text.md")));
    println!("{:#?}", load_md_doc(include_str!("keywords/keywords.md")));
}
