/// 3216. 交换后字典序最小的字符串
// 给你一个仅由数字组成的字符串 s，在最多交换一次 相邻 且具有相同 奇偶性 的数字后，返回可以得到的
// 字典序最小的字符串
// 如果两个数字都是奇数或都是偶数，则它们具有相同的奇偶性。例如，5 和 9、2 和 4 奇偶性相同，而 6 和 9 奇偶性不同。

pub fn get_smallest_string(s: String) -> String {
    let mut ans = String::new();

    let list = s.chars().into_iter().collect::<Vec<char>>();

    for i in 0..(list.len() - 1) {} 

    ans
}

#[test]
fn test_1() {
    assert_eq!(
        get_smallest_string(String::from("45320")),
        String::from("43520")
    )
}

#[test]
fn test_2() {
    assert_eq!(
        get_smallest_string(String::from("001")),
        String::from("001")
    )
}
