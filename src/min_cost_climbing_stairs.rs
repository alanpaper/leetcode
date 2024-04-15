/// 746. 使用最小花费爬楼梯
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut stack = vec![0; cost.len()];
    stack[0] = cost[0];
    stack[1] = (cost[0] + cost[1]).min(cost[1]);
    for index in 2..cost.len() {
        stack[index] = stack[index - 2].min(stack[index - 1]) + cost[index];
    }
    stack[cost.len() - 1].min(stack[cost.len() - 2])
}

#[test]
fn test() {
    assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
}

#[test]
fn test_1() {
    assert_eq!(
        min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}
