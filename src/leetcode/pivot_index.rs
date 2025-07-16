/// 724. 寻找数组的中心下标
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let total: i32 = nums.iter().sum();
    let mut total_left = 0;
    for i in 0..nums.len() {
        if i > 0 {
            total_left += nums[i - 1];
        }
        let right_total = total - total_left - nums[i];
        if total_left == right_total {
            return i as i32;
        }
    }
    -1
}

#[test]
fn test_1() {
    assert_eq!(pivot_index(vec![2, 1, -1]), 0);
}
