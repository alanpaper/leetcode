pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut min_total = 1000;
    let mut abs_min = 0;

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            for k in (j + 1)..nums.len() {
                if (nums[i] + nums[j] + nums[k] - target).abs() < abs_min {
                    min_total = nums[i] + nums[j] + nums[k];
                    abs_min = (nums[i] + nums[j] + nums[k] - target).abs();
                }
            }
        }
    }

    min_total
}
