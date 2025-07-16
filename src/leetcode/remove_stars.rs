/// 2390. 从字符串中移除星号
///
/// 给你一个包含若干星号 * 的字符串 s 。
/// 在一步操作中，你可以：
/// 选中 s 中的一个星号。
/// 移除星号 左侧 最近的那个 非星号 字符，并移除该星号自身。
/// 返回移除 所有 星号之后的字符串。

pub fn remove_stars(s: String) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if c != '*' {
            res.push(c);
        } else {
            res.pop();
        }
    }
    res
}

#[test]
fn test_1() {
    assert_eq!(
        remove_stars(String::from("leet**cod*e")),
        String::from("lecoe")
    );
}
