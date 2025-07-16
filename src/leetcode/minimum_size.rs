// 1760. 袋子里最少数目的球

fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    let mut max_stack = vec![];
    for n in nums {
        max_stack.push(n);
    }

    for _ in 0..max_operations {

        if let Some(x) = max_stack.pop() {
            let avg = x / 2;
            max_stack.push(avg);
            max_stack.push(x - avg);
            println!("{:?}", max_stack);
        }
    }

    max_stack.pop().unwrap()
}


#[test]
fn test1() {
    // nums = [9], maxOperations = 2
    assert_eq!(minimum_size(vec![9], 2), 3)
}
#[test]
fn test2() {
    // nums = [2,4,8,2], maxOperations = 4
    assert_eq!(minimum_size(vec![2,4,8,2], 4), 2)
}
#[test]
fn test3() {
    // nums = [2,4,8,2], maxOperations = 4
    assert_eq!(minimum_size(vec![7,17], 2), 7)
}