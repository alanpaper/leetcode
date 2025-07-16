/// 2443. 翻转之后的数字和
///
/// 给你一个 非负 整数 num 。如果存在某个 非负 整数 k 满足 k + reverse(k) = num  ，则返回 true ；否则，返回 false 。

/// reverse(k) 表示 k 反转每个数位后得到的数字。
///
const NUMS: [i32; 6] = [1, 10, 100, 1000, 10000, 100000];
pub fn sum_of_number_and_reverse(num: i32) -> bool {
    let mut ans = false;
    for i in 0..(num + 1) {
        let mut reverse = 0;
        let nums = reverse_num(i);
        for n in nums.iter().enumerate() {
            reverse += n.1 * NUMS[nums.len() - 1 - n.0];
        }
        if i + reverse == num {
            ans = true;
            break;
        }
    }
    ans
}

fn reverse_num(num: i32) -> Vec<i32> {
    let mut nums = vec![];
    for n in NUMS {
        if num / n >= 1 {
            match nums.last() {
                Some(&x) => {
                    nums.push((num % (n * 10) - x) / n);
                }
                None => {
                    nums.push(num % 10);
                }
            }
        } else {
            break;
        }
    }
    nums
}

#[test]
fn sum_of_number_and_reverse_test() {
    assert_eq!(sum_of_number_and_reverse(0), true);
}
#[test]
fn sum_of_number_and_reverse_test1() {
    assert_eq!(sum_of_number_and_reverse(6), true);
}
#[test]
fn sum_of_number_and_reverse_test2() {
    assert_eq!(sum_of_number_and_reverse(181), true);
}
