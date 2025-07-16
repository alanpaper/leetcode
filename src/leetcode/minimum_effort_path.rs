/// 1631. 最小体力消耗路径
pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let last_x = heights[0].len();
    let last_y = heights.len();

    let mut x = 0;
    let mut y = 0;
}

#[test]
fn test() {
    assert_eq!(
        minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]]),
        2
    )
}
#[test]
fn test_1() {
    assert_eq!(
        minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]),
        1
    )
}
