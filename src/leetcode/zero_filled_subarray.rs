/// 2348. 全是0子数组的数目


pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    let mut queue = vec![];
    for i in 0..nums.len() {
        if nums[i] == 0 {
            queue.push(0);
        } else {
            let size = queue.len() as i64;
            ans += (size + 1) * size / 2;
            queue.clear();
        }
    }
    if !queue.is_empty() {
        let size = queue.len() as i64;
        ans += (size + 1) * size / 2;
        queue.clear();
    }
    ans
}


#[test]
fn test_1() {
    assert_eq!(zero_filled_subarray(vec![1,3,0,0,2,0,0,4]), 6);
}
#[test]
fn test_2() {
    assert_eq!(zero_filled_subarray(vec![0,0,0,2,0,0]), 9);
}

#[test]
fn test_3() {
    assert_eq!(zero_filled_subarray(vec![2,10,2019]), 0);
}