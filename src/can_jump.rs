/// 55. 跳跃游戏
///

pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }

    let last: i32 = nums.len() as i32 - 1;
    let frist = nums[0];

    if frist > last {
        return true;
    }
    let mut zero_arr = vec![];
    for n in nums.iter().enumerate() {
        if n.1 == &0 {
            zero_arr.push(n.0);
        }
    }

    let mut cans = vec![];
    for zero in zero_arr {
        let mut n = zero;
        let mut distance = 1;
        let mut curr_can = false;
        if n == nums.len() - 1 {
            while n > 0 {
                if nums[n - 1] >= distance {
                    curr_can = true;
                    break;
                } else {
                    n = n - 1;
                    distance = distance + 1;
                }
            }
        } else {
            while n > 0 {
                if nums[n - 1] > distance {
                    curr_can = true;
                    break;
                } else {
                    n = n - 1;
                    distance = distance + 1;
                }
            }
        }
        cans.push(curr_can);
    }

    if cans.contains(&false) {
        return false;
    }

    true
}

#[test]
fn test_1() {
    assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true)
}

#[test]
fn test_2() {
    assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false)
}

#[test]
fn test_3() {
    assert_eq!(can_jump(vec![2, 5, 0, 0]), true)
}

fn test() {
    let tets: Vec<i32> = vec![1, 1, 1];

    for i in tets {
        if (i * i) as i64 > 9i64 {}
    }
}
