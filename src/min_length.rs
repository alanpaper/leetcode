/// 2696. 删除子串后的字符串最小长度
/// 给你一个仅由 大写 英文字符组成的字符串 s 。
/// 你可以对此字符串执行一些操作，在每一步操作中，你可以从 s 中删除 任一个 "AB" 或 "CD" 子字符串。
/// 通过执行操作，删除所有 "AB" 和 "CD" 子串，返回可获得的最终字符串的 最小 可能长度。
/// 注意，删除子串后，重新连接出的字符串可能会产生新的 "AB" 或 "CD" 子串。
pub fn min_length(s: String) -> i32 {
    let mut stack = vec![];

    for char in s.chars() {
        match stack.last() {
            Some(&last) => {
                if (last == 'A' && char == 'B') || (last == 'C' && char == 'D') {
                    stack.pop();
                } else {
                    stack.push(char);
                }
            }
            None => stack.push(char),
        }
    }

    stack.len() as i32
}

#[test]
fn test_min_length() {
    let num = min_length("ABFCACDB".to_owned());
    assert_eq!(num, 2);
}
