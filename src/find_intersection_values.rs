use std::cmp::Ordering;

/// 2956. 找到两个数组的公共元素
pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut num1 = nums1.clone();
    num1.sort();
    let mut num2 = nums2.clone();
    num2.sort();

    let mut left = 0;
    let mut right = 0;
    for num in nums1 {
        if split_find(num, &num2) {
            left += 1;
        }
    }
    for num in nums2 {
        if split_find(num, &num1) {
            right += 1;
        }
    }
    vec![left, right]
}

fn split_find(num: i32, arr: &Vec<i32>) -> bool {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let middle = (left + right) / 2;
        match arr[middle].cmp(&num) {
            Ordering::Less => {
                left = middle + 1;
            }
            Ordering::Equal => {
                return true;
            }
            Ordering::Greater => {
                right = middle;
            }
        }
    }
    false
}

#[test]
fn test_1() {
    let num1 = vec![4, 3, 2, 3, 1];
    let num2 = vec![2, 2, 5, 2, 3, 6];
    assert_eq!(find_intersection_values(num1, num2), vec![3, 4]);
}
