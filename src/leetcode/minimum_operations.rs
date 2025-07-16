/// 2844. 生成特殊数字的最小操作
/// 给你一个下标从 0 开始的字符串 num ，表示一个非负整数。
/// 在一次操作中，您可以选择 num 的任意一位数字并将其删除。请注意，如果你删除 num 中的所有数字，则 num 变为 0。
/// 返回最少需要多少次操作可以使 num 变成特殊数字。
/// 如果整数 x 能被 25 整除，则该整数 x 被认为是特殊数字。
pub fn minimum_operations(num: String) -> i32 {
    let mut nums = num.chars().into_iter().collect::<Vec<char>>();
    if num == String::from("0") {
        return 0;
    }
    if nums.len() == 1 {
        return 1;
    }
    let nums_enum = vec!["25".to_string(), "50".to_string(), "75".to_string()];
    if nums.len() == 2 {
        if nums_enum.contains(&num) {
            return 0;
        } else if nums[1] == '0' {
            return 1;
        } else {
            return 2;
        }
    }
    nums.reverse();
    let result = vec![('0', '0'), ('0', '5'), ('5', '2'), ('5', '7')];
    let mut ans = nums.len();
    let mut is_pre_zero = false;
    let mut is_break = false;
    for n in result {
        let mut find = false;
        let mut total = 0;
        for y in &nums {
            if *y == n.0 && !find {
                find = true;
                if *y == '0' {
                    is_pre_zero = true
                }
            } else if *y == n.1 && find {
                is_break = true;
                break;
            } else {
                total += 1;
            }
        }
        ans = ans.min(total);
    }

    if !is_break {
        if is_pre_zero {
            return nums.len() as i32 - 1;
        }
        return nums.len() as i32;
    }
    ans as i32
}

#[test]
fn test_1() {
    assert_eq!(minimum_operations("2245047".to_string()), 2);
}

#[test]
fn test_2() {
    assert_eq!(minimum_operations("2908305".to_string()), 3);
}

#[test]
fn test_3() {
    assert_eq!(minimum_operations("53478".to_string()), 5);
}

#[test]
fn test_4() {
    assert_eq!(minimum_operations("820366".to_string()), 5);
}
