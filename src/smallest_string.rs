/// 2734. 执行字串操作后的字典序最小字符串
///
/// 给你一个仅由小写英文字母组成的字符串 s 。在一步操作中，你可以完成以下行为：
// 选择 s 的任一非空子字符串，可能是整个字符串，接着将字符串中的每一个字符替换为英文字母表中的前一个字符。例如，'b' 用 'a' 替换，'a' 用 'z' 替换。
// 返回执行上述操作 恰好一次 后可以获得的 字典序最小 的字符串。
// 子字符串 是字符串中的一个连续字符序列。
// 现有长度相同的两个字符串 x 和 字符串 y ，在满足 x[i] != y[i] 的第一个位置 i 上，
/// 如果  x[i] 在字母表中先于 y[i] 出现，则认为字符串 x 比字符串 y 字典序更小 。

pub fn smallest_string(s: String) -> String {
    let mut str = s.into_bytes();
    let len = str.len();
    for i in 0..len {
        if str[i] > b'a' {
            for j in i..len {
                if str[j] > b'a' {
                    str[j] = str[j] - 1;
                } else {
                    break;
                }
            }
            return unsafe { String::from_utf8_unchecked(str) };
        }
    }
    str[len - 1] = b'z';
    return unsafe { String::from_utf8_unchecked(str) };
}

#[test]
fn test_0() {
    assert_eq!(
        smallest_string(String::from("aaaaaa")),
        String::from("aaaaaz")
    )
}

#[test]
fn test_1() {
    assert_eq!(
        smallest_string(String::from("acbbc")),
        String::from("abaab")
    )
}

#[test]
fn test_3() {
    assert_eq!(
        smallest_string(String::from("cbabc")),
        String::from("baabc")
    )
}

#[test]
fn test_2() {
    assert_eq!(
        smallest_string(String::from("leetcode")),
        String::from("kddsbncd")
    )
}
