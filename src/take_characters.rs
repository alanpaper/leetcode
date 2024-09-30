use std::collections::{HashMap, VecDeque};

/// 2516. 每种字符至少取K个
/// 给你一个由字符 'a'、'b'、'c' 组成的字符串 s 和一个非负整数 k 。
/// 每分钟，你可以选择取走 s 最左侧 还是 最右侧 的那个字符。
/// 你必须取走每种字符 至少 k 个，返回需要的 最少 分钟数；如果无法取到，则返回 -1。
pub fn take_characters(s: String, k: i32) -> i32 {
    let mut vs = VecDeque::new();
    for c in s.chars() {
        vs.push_back(c);
    }

    0
}

#[test]
fn test_1() {
    assert_eq!(take_characters(String::from("aabaaaacaabc"), 2), 8);
}
#[test]
fn test_2() {
    assert_eq!(take_characters(String::from("a"), 1), -1);
}
