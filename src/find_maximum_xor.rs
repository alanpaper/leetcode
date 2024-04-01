/// 421.数组中的两个树的最大异或值

pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for x in 0..(nums.len() - 1) {
        for y in (x + 1)..nums.len() {
            ans = ans.max(nums[x] ^ nums[y]);
        }
    }
    ans
}

#[test]
fn test() {
    assert_eq!(find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28)
}
