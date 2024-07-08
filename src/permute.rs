/// 46. 全排列
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // !TODO
    vec![vec![]]
}

#[test]
fn test_1() {
    assert_eq!(permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]])
}

#[test]
fn test_2() {
    assert_eq!(permute(vec![1]), vec![vec![1]])
}
