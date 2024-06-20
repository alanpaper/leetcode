use std::collections::HashMap;

/// 670. 最大交换
/// 给定一个非负整数，你至多可以交换一次数字中的任意两位。返回你能得到的最大值。
pub fn maximum_swap(num: i32) -> i32 {
    let mut nums = vec![];
    let mut ans = num;
    while ans > 0 {
        let n = ans % 10;
        ans /= 10;
        nums.push(n);
    }
    nums.reverse();
    let mut mp = HashMap::new();
    for (index, n) in nums.iter().enumerate() {
        mp.entry(n).or_insert(Vec::new()).push(index);
    }
    let mut sorted_mp: Vec<_> = mp.iter().collect();
    sorted_mp.sort_by(|a, b| b.0.cmp(&a.0));
    let mut p = 0;
    let mut q = 0;
    let mut index = 0;
    for n in sorted_mp {
        let current_num = **n.0;
        let mut is_break = false;
        for _ in n.1 {
            if nums[index] != current_num {
                p = index;
                q = *n.1.last().unwrap();
                is_break = true;
                break;
            }
            index += 1;
        }
        if is_break {
            break;
        }
    }
    nums.swap(p, q as usize);
    nums.reverse();
    let mut res = 0;
    let mut speed = 1;
    for n in nums {
        res += n * speed;
        speed *= 10;
    }
    res as i32
}

#[test]
fn test_1() {
    assert_eq!(maximum_swap(1993), 9913);
}

#[test]
fn test_6() {
    assert_eq!(maximum_swap(2756), 7256);
}

#[test]
fn test_2() {
    assert_eq!(maximum_swap(40567), 70564);
}

#[test]
fn test_3() {
    assert_eq!(maximum_swap(9973), 9973);
}
#[test]
fn test_4() {
    assert_eq!(maximum_swap(98368), 98863);
}
