use std::collections::HashMap;

/// 2981. 寻找出出现至少三次的最长特殊子字符串 1
/// 给你一个仅由小写英文字母组成的字符串 s 。
/// 如果一个字符串仅由单一字符组成，那么它被称为 特殊 字符串。例如，字符串 "abc" 不是特殊字符串，而字符串 "ddd"、"zz" 和 "f" 是特殊字符串。
/// 返回在 s 中出现 至少三次 的 最长特殊子字符串 的长度，如果不存在出现至少三次的特殊子字符串，则返回 -1 。
/// 子字符串 是字符串中的一个连续 非空 字符序列。
///
///
/// [abcabcabcaaaaa]

pub fn maximum_length(s: String) -> i32 {
    let mut map: HashMap<char, Vec<Vec<char>>> = HashMap::new();
    let mut min_queue: Vec<char> = vec![];
    for c in s.chars() {
        if let Some(&last_c) = min_queue.last() {
            println!("{:?} =======", min_queue);
            if last_c == c {
                min_queue.push(c);
            } else {
                if min_queue.len() == 1 {
                    map.entry(last_c)
                        .and_modify(|f| f.push(min_queue.clone()))
                        .or_insert(vec![vec![last_c]]);
                } else {
                    map.entry(last_c)
                        .and_modify(|f| f.push(min_queue.clone()))
                        .or_insert(vec![min_queue.clone()]);
                }
                min_queue.clear();
                min_queue.push(c);
            }
        } else {
            min_queue.push(c);
        }
    }
    if !min_queue.is_empty() {
        map.entry(*min_queue.last().unwrap())
            .and_modify(|f| f.push(min_queue.clone()))
            .or_insert(vec![min_queue.clone()]);
        min_queue.clear();
    }

    println!("{:?}", map);

    let mut ans = -1;
    for (_, chars) in map.iter() {
        ans = ans.max(dealwidth_ans(chars));
    }

    ans
}

fn dealwidth_ans(ans: &Vec<Vec<char>>) -> i32 {
    let mut map = HashMap::new();
    ans.iter().for_each(|f| {
        map.entry(f.len()).and_modify(|f| *f += 1).or_insert(1);
    });
    let mut ans = vec![];
    map.iter().for_each(|f| {
        ans.push(f);
    });
    ans.sort_by(|a, b| b.0.cmp(a.0));
    let mut ans_max = 0;

    println!("{:?}=====ans", ans);

    let mut total = 0;
    let mut max_queue = vec![];
    for f in ans.iter() {
        total += *f.1 * *f.0;
        if (*f.0 >= 3 && *f.1 >= 3) || *f.1 >= 3 {
            ans_max = ans_max.max(*f.0);
        } else if *f.0 > 3 && *f.1 == 2 {
            ans_max = ans_max.max(*f.0 - 1);
            max_queue.push(*f.0);
        } else if *f.0 >= 3 {
            ans_max = ans_max.max(*f.0 - 3 + 1);
            max_queue.push(*f.0);
        } else if *f.0 == 2 {
            max_queue.push(*f.0);
        }
    }
    max_queue.sort_by(|a, b| b.cmp(a));

    if max_queue.len() >= 2 {
        ans_max = ans_max.max(max_queue[1]);
    }

    if total >= 3 && ans_max == 0 {
        return 1;
    }

    if ans_max == 0 {
        return -1;
    }
    ans_max as i32
}

#[test]
fn test_1() {
    assert_eq!(maximum_length(String::from("aaaa")), 2)
}

#[test]
fn test_2() {
    assert_eq!(maximum_length(String::from("ejcemvvvvvvmmvvvvaaa")), 4)
}

#[test]
fn test_3() {
    assert_eq!(
        maximum_length(String::from(
            "ddeddeyyyceedeeeeceedceddecdcdddcccdeeededceeeddde"
        )),
        3
    )
}

/// 3176. 求出最长好子序列 I
/// 给你一个整数数组 nums 和一个 非负 整数 k 。
/// 如果一个整数序列 seq 满足在下标范围 [0, seq.length - 2] 中 最多只有 k 个下标 i 满足 seq[i] != seq[i + 1] ，
/// 那么我们称这个整数序列为 好 序列。
/// 请你返回 nums 中 好子序列的最长长度
/// 子序列:是可以通过从另一个数组删除或不删除某些元素，但不更改其余元素的顺序得到的数组。
/// 
pub fn maximum_length_i(nums: Vec<i32>, k: i32) -> i32 {
    0
}

#[test]
fn test_i_1() {
    assert_eq!(maximum_length_i(vec![1, 2, 1, 1, 3], 2), 4)
}

#[test]
fn test_i_2() {
    assert_eq!(maximum_length_i(vec![1, 2, 3, 4, 5, 1], 0), 2)
}
