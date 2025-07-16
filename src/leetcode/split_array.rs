/// 410. 分割数组的最大值
pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut low: i32 = *nums.iter().max().unwrap();
    let mut high: i32 = nums.iter().sum();

    while low < high {
        let mid = (low + high) / 2;

        let mut count = 0;
        let mut sub_sum = 0;
        for n in &nums {
            sub_sum += *n;
            if sub_sum > mid {
                count += 1;
                sub_sum = *n;
            }
        }
        count += 1;
        if count > k {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}

#[test]
fn tests() {
    assert_eq!(split_array(vec![7, 2, 5, 10, 8], 2), 18);
}
