/// 2216. 美化数组的最少删除数
/// nums.length 为偶数
/// 对所有满足 i % 2 == 0 的下标 i ，nums[i] != nums[i + 1] 均成立
/// 注意，空数组同样认为是美丽数组。
///
pub fn min_deletion(nums: Vec<i32>) -> i32 {
    let mut nums_copy = nums.clone();
    let mut index = 0;

    let mut ans = 0;

    while index < nums_copy.len() - 1 {
        let mut remove = false;
        if index % 2 == 0 {
            if nums_copy[index] == nums_copy[index + 1] {
                ans += 1;
                remove = true;
                nums_copy.remove(index + 1);
            }
        }

        if !remove {
            index += 1;
        }
    }

    if nums_copy.len() % 2 != 0 {
        ans += 1;
    }

    ans
}

#[test]
fn test() {
    assert_eq!(min_deletion(vec![1, 1, 2, 3, 5]), 1);
}

#[test]
fn test_1() {
    assert_eq!(min_deletion(vec![1, 1, 2, 2, 3, 3]), 2);
}
