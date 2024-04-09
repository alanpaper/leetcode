use std::collections::HashMap;

/// 2342. 数位和相等数对的最大和
pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    let mut max_val = -1;

    for c in nums {
        let mut total = 0;
        let mut max_num = c;
        while max_num > 0 {
            total += max_num % 10;
            max_num = max_num / 10;
        }
        hash_map
            .entry(total)
            .and_modify(|x| {
                max_val = max_val.max(*x + c);
                *x = c.max(*x);
            })
            .or_insert(c);
    }
    max_val
}

#[test]
fn test() {
    assert_eq!(
        maximum_sum(vec![
            229, 398, 269, 317, 420, 464, 491, 218, 439, 153, 482, 169, 411, 93, 147, 50, 347, 210,
            251, 366, 401
        ]),
        973
    )
}

// #[test]
// fn test_1() {
//     assert_eq!(maximum_sum(vec![10, 12, 19, 14]), -1)
// }
