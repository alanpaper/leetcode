/// 2656. K个元素的最大和
///
pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    match nums.iter().max() {
        Some(m)=> {
          (m + m + k - 1) * k / 2
        },
        None=>{0}
    }
}

#[test]
fn _test() {
    assert_eq!(maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
}
