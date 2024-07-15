/// 2789. 合并后数组中的最大元素
pub fn max_array_value(nums: Vec<i32>) -> i64 {
    let max_total = dfs(nums);

    fn dfs(nums: Vec<i32>) -> i64 {
        let mut stack = vec![];
        let mut stack_sums = vec![];
        let mut max_total: i32 = 0;
        let mut total: i32 = 0;
        for num in &nums {
            if stack.last() <= Some(num) && stack.len() > 0 {
                total += *num;
                stack.push(*num);
            } else if stack.len() == 0 {
                total += *num;
                stack.push(*num);
            } else {
                stack = vec![];
                stack.push(*num);
                stack_sums.push(total);
                max_total = max_total.max(total);
                total = 0;
            }
        }
        stack_sums.push(stack[0]);
        if stack.len() > 0 {
            max_total = max_total.max(stack[0]);
        }
        max_total = max_total.max(dfs(stack_sums) as i32);
        max_total as i64
    }

    max_total
}

#[test]
fn max_array_value_test() {
    assert_eq!(max_array_value(vec![2, 3, 7, 9, 3]), 21);
}
#[test]
fn max_array_value_test_1() {
    assert_eq!(max_array_value(vec![5, 3, 3]), 11);
}
