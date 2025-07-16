use std::collections::HashMap;
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
    let mut frequency = 0;
    let mut i_index: i32 = 0;

    let mut s2_chars: HashMap<char, i32> = HashMap::new();
    for char in s2.chars() {
        s2_chars.entry(char).or_insert(0);
    }

    loop {
        println!("{:?}", s2_chars);
        for (i, char) in s1.chars().into_iter().enumerate() {
            match s2_chars.get(&char) {
                Some(val) => {
                    if *val == 0 {
                      s2_chars.entry(char).or_insert(1);
                    } else {
                      
                    }
                }
                None => {
                    continue;
                }
            }
        }

        if frequency < n1 {
            frequency += 1;
        } else {
            break;
        }

        if s2_chars.len() == 0 {
            break;
        }
    }

    for (i, char) in s1.chars().into_iter().enumerate() {
        match s2_chars.get(0) {
            Some(c2) => {
                if char == *c2 {
                    i_index += i as i32 + 1;
                    break;
                }
            }
            None => {}
        }
    }

    println!("{:?}--{:?}", i_index, i_start);

    s1.len() as i32 * n1 / (i_index - i_start) / n2
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
