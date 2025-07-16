pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    0
}

#[test]
fn test_count_paths() {
    assert_eq!(count_paths(
        6,
        vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![3, 6]]
    ))
}
