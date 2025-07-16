/// 3163. 压缩字符串III
/// 给你一个字符串 word，请你使用以下算法进行压缩:
// 从空字符串 comp 开始。当 word 不为空 时，执行以下操作：
// 移除 word 的最长单字符前缀，该前缀由单一字符 c 重复多次组成，且该前缀长度 最多 为 9 。
// 将前缀的长度和字符 c 追加到 comp 。
// 返回字符串 comp 。

pub fn compressed_string(word: String) -> String {
    let mut queue = vec![];
    let mut ans = String::new();
    for c in word.chars() {
        if queue.is_empty() {
            queue.push(c);
        } else if let Some(last) = queue.last() {
            if *last == c {
                if queue.len() < 9 {
                    queue.push(c);
                } else {
                    ans += &queue.len().to_string();
                    ans += &last.to_string();
                    queue.clear();
                    queue.push(c);
                }
            } else {
                ans += &queue.len().to_string();
                ans += &last.to_string();
                queue.clear();
                queue.push(c);
            }
        }
    }
    if !queue.is_empty() {
        ans += &queue.len().to_string();
        ans += &queue.last().unwrap().to_string();
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(
        compressed_string(String::from("abcde")),
        String::from("1a1b1c1d1e")
    );
}

#[test]
fn test_2() {
    assert_eq!(
        compressed_string(String::from("aaaaaaaaaaaaaabb")),
        String::from("9a5a2b")
    );
}
