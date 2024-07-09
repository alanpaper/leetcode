use std::collections::HashMap;

/// 2085. 统计出现过一次的公共字符串
pub fn count_words(words1: Vec<&str>, words2: Vec<&str>) -> i32 {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for word in words1 {
        map1.entry(word)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    for word in words2 {
        map2.entry(word)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut total = 0;
    for (str, &count) in map1.iter() {
        if count == 1 && map2.get(str) == Some(&1) {
            total += 1;
        }
    }

    total
}

#[test]
fn test_count_words() {
    let words1 = vec!["leetcode", "is", "amazing", "as", "is"];
    let words2 = vec!["amazing", "leetcode", "is"];
    assert_eq!(count_words(words1, words2), 2);
}

// words1 = ["leetcode","is","amazing","as","is"], words2 = ["amazing","leetcode","is"]
