use super::*;

#[derive(Debug)]
struct Output2 {
    pub a: char,
    pub b: Vec<char>,
    pub c: char,
    pub range: std::ops::Range<usize>,
}

/// `ab{1,3}c`
fn parse_output2(state: YState) -> YResult<Output2> {
    let start = state.start_offset;
    let (state, a) = state.match_char('a')?;
    let (state, b) = state.match_repeat_m_n(1, 3, |state| state.match_char('b'))?;
    let (state, c) = state.match_char('c')?;
    let range = start..state.start_offset;
    state.finish(Output2 { a, b, c, range })
}

#[test]
fn test_output_1() {
    println!("{:#?}", parse_output2(YState::new("ac")));
    println!("{:#?}", parse_output2(YState::new("abc")));
    println!("{:#?}", parse_output2(YState::new("abbc")));
    println!("{:#?}", parse_output2(YState::new("abbbc")));
    println!("{:#?}", parse_output2(YState::new("abbbbc")));
}