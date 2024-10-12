/// 3174.清除数字
///  给你一个字符串 s 。
/// 你的任务是重复以下操作删除 所有 数字字符：
/// 删除 第一个数字字符 以及它左边 最近 的 非数字 字符。
/// 请你返回删除所有数字字符以后剩下的字符串。
pub fn clear_digits(s: String) -> String {
    let mut queue = vec![];
    for c in s.chars() {
        if c as u8 >= 48 && c as u8 <= 57 {
            queue.pop();
        } else {
            queue.push(c);
        }
    }
    queue.iter().collect()
}

#[test]
fn test_1() {
    assert_eq!(clear_digits(String::from("abc")), "abc".to_string())
}
#[test]
fn test_2() {
    assert_eq!(clear_digits(String::from("abc12")), "a".to_string())
}
