pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut pre = 0;
    let mut max_total = nums[0];

    for num in nums {
        pre = num.max(num + pre);
        max_total = max_total.max(pre);
    }

    max_total
}

#[test]
fn test() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6)
}
