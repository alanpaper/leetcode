/// 2414. 最长的字母序连续子字符串的长度
/// 字母序连续字符串 是由字母表中连续字母组成的字符串。换句话说，
/// 字符串 "abcdefghijklmnopqrstuvwxyz" 的任意子字符串都是 字母序连续字符串 。
/// 例如，"abc" 是一个字母序连续字符串，而 "acb" 和 "za" 不是。
/// 给你一个仅由小写英文字母组成的字符串 s ，返回其 最长 的 字母序连续子字符串 的长度。
pub fn longest_continuous_substring(s: String) -> i32 {
    let mut res = 0;
    let mut ans = 1;
    let mut pre = None;
    for c in s.chars() {
        if let Some(p) = pre {
            let current = c as u32;
            if p + 1 == current {
                ans += 1;
            } else {
                res = ans.max(res);
                ans = 1;
            }
        }
        pre = Some(c as u32)
    }
    res = ans.max(res);
    res
}

#[test]
fn test_1() {
    assert_eq!(longest_continuous_substring(String::from("abacaba")), 2);
}
#[test]
fn test_2() {
    assert_eq!(longest_continuous_substring(String::from("abcde")), 5);
}
