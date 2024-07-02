/// 3115. 质数的最大距离
pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    let mut index = 0;
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right && index <= nums.len() {
        if !primes.contains(&nums[left]) {
            left += 1;
        }
        if !primes.contains(&nums[right]) {
            right -= 1;
        }
        index += 1;
    }
    (right - left) as i32
}

#[test]
fn test_1() {
    let nums = vec![4, 2, 9, 5, 3];
    assert_eq!(maximum_prime_difference(nums), 3);
}

#[test]
fn test_2() {
    let nums = vec![4, 8, 2, 8];
    assert_eq!(maximum_prime_difference(nums), 0);
}
#[test]
fn test_3() {
    let nums = vec![3, 49, 70, 9];
    assert_eq!(maximum_prime_difference(nums), 0);
}
