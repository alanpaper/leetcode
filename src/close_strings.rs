use std::collections::HashSet;
/// 1657. 确定两个字符串是否接近

pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut map_1 = [0; 26];
    let mut map_2 = [0; 26];

    let mut set_1 = HashSet::new();
    let mut set_2 = HashSet::new();

    word1.chars().for_each(|c| {
        map_1[(c as i64 - 97) as usize] += 1;
        set_1.insert(c);
    });
    word2.chars().for_each(|c| {
        map_2[(c as i64 - 97) as usize] += 1;
        set_2.insert(c);
    });

    let ans_chars = set_1.difference(&set_2);

    let mut ans_char = true;
    for _ in ans_chars {
        ans_char = false;
    }

    let mut ans = true;
    for num in map_1 {
        if map_2.contains(&num) {
            continue;
        } else {
            ans = false;
        }
    }
    for num in map_2 {
        if map_1.contains(&num) {
            continue;
        } else {
            ans = false;
        }
    }

    if !ans_char {
        return false;
    }

    ans
}

#[test]
fn test() {
    assert_eq!(
        close_strings(String::from("abc"), String::from("bca")),
        true
    )
}

#[test]
fn test_1() {
    assert_eq!(close_strings(String::from("a"), String::from("aa")), false)
}

#[test]
fn test_2() {
    assert_eq!(
        close_strings(String::from("cabbba"), String::from("abbccc")),
        true
    )
}
