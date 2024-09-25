use std::collections::HashSet;
/// 2306. 公司命名
/// 给你一个字符串数组 ideas 表示在公司命名过程中使用的名字列表。公司命名流程如下：
/// 从 ideas 中选择 2 个 不同 名字，称为 ideaA 和 ideaB 。
/// 交换 ideaA 和 ideaB 的首字母。
/// 如果得到的两个新名字 都 不在 ideas 中，那么 ideaA ideaB（串联 ideaA 和 ideaB ，中间用一个空格分隔）是一个有效的公司名字。
/// 否则，不是一个有效的名字。
/// 返回 不同 且有效的公司名字的数目。

pub fn distinct_names(ideas: Vec<String>) -> i64 {
    let mut name_set = HashSet::new();
    let mut ans_set = HashSet::new();
    for i in &ideas {
        name_set.insert(i.clone());
    }
    for i in 0..ideas.len() {
        for j in 0..ideas.len() {
            let idea_a = &ideas[i];
            let idea_b = &ideas[j];
            if idea_a != idea_b {
                let idea_new_a =
                    format!("{}{}", idea_b.get(0..1).unwrap(), idea_a.get(1..).unwrap());
                let idea_new_b =
                    format!("{}{}", idea_a.get(0..1).unwrap(), idea_b.get(1..).unwrap());
                if !name_set.contains(&idea_new_a) && !name_set.contains(&idea_new_b) {
                    ans_set.insert(format!("{} {}", idea_new_a, idea_new_b));
                }
            }
        }
    }
    ans_set.iter().len() as i64
}

#[test]
fn test_1() {
    assert_eq!(
        distinct_names(vec![
            "coffee".to_string(),
            "donuts".to_string(),
            "time".to_string(),
            "toffee".to_string()
        ]),
        6
    );
}

#[test]
fn test_2() {
    assert_eq!(
        distinct_names(vec!["lack".to_string(), "back".to_string()]),
        0
    );
}
