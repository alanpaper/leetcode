use std::collections::HashMap;

/// 2244. 完成所有任务需要的最少轮数
///
/// ### 解
/// 除了1以外 其他数量都能被2和3已不同的数量加出来
pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let mut task_map = HashMap::new();
    for task in tasks {
        task_map.entry(task).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut total = 0;
    for (_key, val) in &task_map {
        if *val == 1 {
            return -1;
        }
        total += (*val + 2) / 3;
    }
    total
}

#[test]
fn test() {
    assert_eq!(minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]), 4);
}
#[test]
fn test_1() {
    assert_eq!(minimum_rounds(vec![2, 3, 3]), -1);
}



