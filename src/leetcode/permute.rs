/// 46. 全排列
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {

    let mut total = 0;
    for i in 1..=nums.len() {
        total *= i;
    }

    while total > 0 {
        let mut middle = vec![0; nums.len()];
        total -= 1;
    }

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
