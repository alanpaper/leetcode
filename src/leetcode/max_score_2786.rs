/// 2786. 访问数组中的位置使分数最大
/// 给你一个下标从 0 开始的整数数组 nums 和一个正整数 x 。
/// 你 一开始 在数组的位置 0 处，你可以按照下述规则访问数组中的其他位置：
/// 如果你当前在位置 i ，那么你可以移动到满足 i < j 的 任意 位置 j 。
/// 对于你访问的位置 i ，你可以获得分数 nums[i] 。
/// 如果你从位置 i 移动到位置 j 且 nums[i] 和 nums[j] 的 奇偶性 不同，那么你将失去分数 x 。
/// 请你返回你能得到的 最大 得分之和。
///
/// 注意 ，你一开始的分数为 nums[0]。
///
///

#[derive(Debug)]
struct CurrentQueue {
    total: i64,
    list: Vec<i32>,
}
impl CurrentQueue {
    fn new() -> Self {
        CurrentQueue {
            total: 0,
            list: vec![],
        }
    }
    fn push(&mut self, num: i32) {
        self.total += num as i64;
        self.list.push(num);
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    fn clear(&mut self) {
        self.list.clear();
        self.total = 0;
    }
}

pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
    let mut total = nums[0] as i64;
    let mut pre = nums[0];
    let mut current_one_queue = CurrentQueue::new();
    current_one_queue.push(nums[0]);
    let mut current_two_queue = CurrentQueue::new();
    for index in 1..nums.len() {
        if (nums[index] - pre).abs() % 2 == 0 {
            if current_two_queue.is_empty() {
                current_one_queue.push(nums[index])
            } else {
                current_two_queue.push(nums[index]);
            }
        } else {
            if current_one_queue.is_empty() && current_two_queue.is_empty() {
                current_one_queue.push(nums[index])
            } else if current_two_queue.is_empty() {
                current_two_queue.push(nums[index])
            } else {
                let num = current_one_queue
                    .total
                    .max(current_two_queue.total - x as i64) as i64;
                println!("{:?}", num);

                total += current_one_queue
                    .total
                    .max(current_two_queue.total - x as i64) as i64;
                println!("current_one_queue = {:?} \n", current_one_queue);
                println!("current_two_queue = {:?} \n", current_two_queue);
                current_one_queue.clear();
                current_two_queue.clear();
                current_one_queue.push(nums[index])
            }
        }
        pre = nums[index]
    }

    if !current_one_queue.is_empty() || !current_two_queue.is_empty() {
        println!("current_one_queue1 = {:?} \n", current_one_queue);
        println!("current_two_queue1 = {:?} \n", current_two_queue);

        total += current_one_queue
            .total
            .max(current_two_queue.total - x as i64) as i64;
    }

    total
}

#[test]
fn test_1() {
    assert_eq!(max_score(vec![2, 3, 6, 1, 9, 2], 5), 13);
}

// #[test]
// fn test_2() {
//     assert_eq!(max_score(vec![2, 4, 6, 8], 3), 20);
// }

// #[test]
// fn test_3() {
//     assert_eq!(
//         max_score(
//             vec![
//                 38, 92, 23, 30, 25, 96, 6, 71, 78, 77, 33, 23, 71, 48, 87, 77, 53, 28, 6, 20, 90,
//                 83, 42, 21, 64, 95, 84, 29, 22, 21, 33, 36, 53, 51, 85, 25, 80, 56, 71, 69, 5, 21,
//                 4, 84, 28, 16, 65, 7
//             ],
//             52
//         ),
//         1545
//     );
// }
