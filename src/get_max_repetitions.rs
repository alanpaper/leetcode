use std::collections::VecDeque;
/// 466. 统计重复个数
/// 定义 str = [s, n] 表示 str 由 n 个字符串 s 连接构成。

/// 例如，str == ["abc", 3] =="abcabcabc" 。
///
/// abcabcdefebdfs
///
/// bf
/// 如果可以从 s2 中删除某些字符使其变为 s1，则称字符串 s1 可以从字符串 s2 获得。

/// 例如，根据定义，s1 = "abc" 可以从 s2 = "abdbec" 获得，仅需要删除加粗且用斜体标识的字符。
/// 现在给你两个字符串 s1 和 s2 和两个整数 n1 和 n2 。由此构造得到两个字符串，其中 str1 = [s1, n1]、str2 = [s2, n2] 。

/// 请你找出一个最大整数 m ，以满足 str = [str2, m] 可以从 str1 获得。
///
///
/// 题解
///
///
///

pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
    if s1 == s2 {
        return n1 / n2;
    }

    let mut i_start = 0;
    let mut i_end = 0;
    let mut i_index: i32 = 0;

    let mut s1_str = String::new();
    for _i in 0..n1 {
        s1_str.push_str(&s1);
    }

    let mut s2_chars = VecDeque::new();
    for char in s2.chars() {
        s2_chars.push_back(char);
    }

    for (i, char) in s1_str.chars().into_iter().enumerate() {
        if char == s2_chars[i_index.try_into().unwrap()] {
            match s2_chars.pop_front() {
                Some(_c) => {
                    if i_index == 0 {
                        i_start = i as i32;
                    }
                    i_index += 1;
                }
                None => println!("{:?}", i_index),
            };
        }
    }

    i_index - i_start
}

#[test]
fn solve_test() {
    let max = get_max_repetitions("baba".to_string(), 11, "baab".to_string(), 1);
    assert_eq!(max, 7);
}
#[test]
fn solve_test_1() {
    let max = get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2);
    assert_eq!(max, 2);
}
