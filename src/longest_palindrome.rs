/// 5. 最长回文子串
/// 给你一个字符串 s，找到 s 中最长的回文子串
pub fn longest_palindrome(s: String) -> String {
    String::new()
}

#[test]
fn tets() {
    assert_eq!(
        longest_palindrome(String::from("babad")),
        String::from("bab")
    );
}

#[test]
fn tets_1() {
    assert_eq!(longest_palindrome(String::from("cbbd")), String::from("bb"));
}
