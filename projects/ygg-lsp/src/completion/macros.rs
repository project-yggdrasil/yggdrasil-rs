use super::*;

pub static COMPLETE_MACROS: SyncLazy<Vec<CompletionItem>> = SyncLazy::new(|| complete_macros());

pub fn complete_macros() -> Vec<CompletionItem> {
    let parsed = load_md_doc(include_str!("macros.md"));
    parsed.iter().map(build_command).collect()
}

pub fn build_command(doc: &DocumentString) -> CompletionItem {
    let cmd = doc.cmd.to_owned();
    let short = doc.short.to_owned();
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: doc.long.to_owned() };
    CompletionItem {
        label: format!("{}", cmd),
        kind: Some(CompletionItemKind::Function),
        detail: Some(short),
        documentation: Some(Documentation::MarkupContent(doc)),
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: Some(format!("{}($1)", cmd)),
        insert_text_format: Some(InsertTextFormat::Snippet),
        insert_text_mode: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    }
}
