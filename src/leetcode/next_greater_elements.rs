/// 503. 下一个更大元素 II
/// 给定一个循环数组 nums （ nums[nums.length - 1] 的下一个元素是 nums[0] ），返回 nums 中每个元素的 下一个更大元素。
/// 数字 x 的 下一个更大的元素 是按数组遍历顺序，这个数字之后的第一个比它更大的数，这意味着你应该循环地搜索它的下一个更大的数。如果不存在，则输出 -1。
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for i in 0..nums.len() {
        let mut j = i + 1;
        let mut next_max = -1;
        let total = nums.len() + i;
        while j < total {
            let current = match nums.get(j) {
                Some(c) => *c,
                None => *nums.get(j - nums.len()).unwrap(),
            };
            if nums[i] < current {
                next_max = current;
                break;
            }
            j += 1;
        }
        ans.push(next_max);
    }
    ans
}

#[test]
fn test_1() {
    let nums = vec![1, 2, 1];
    assert_eq!(next_greater_elements(nums), vec![2, -1, 2])
}
#[test]
fn test_2() {
    let nums = vec![1, 2, 3, 4, 3];
    assert_eq!(next_greater_elements(nums), vec![2, 3, 4, -1, 4])
}
