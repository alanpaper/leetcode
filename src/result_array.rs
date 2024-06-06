use std::cmp::Ordering;
/// 3072. 将元素分配到两个数组中II
pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut arr1 = vec![];
    let mut arr2 = vec![];
    for num in nums {
        if arr1.is_empty() {
            arr1.push(num);
            continue;
        }
        if arr2.is_empty() {
            arr2.push(num);
            continue;
        }
        match greater_count(&arr1, num).cmp(&greater_count(&arr2, num)) {
            Ordering::Less => {
                arr2.push(num);
            }
            Ordering::Greater => {
                arr1.push(num);
            }
            Ordering::Equal => match arr1.len().cmp(&arr2.len()) {
                Ordering::Less => {
                    arr1.push(num);
                }
                Ordering::Greater => {
                    arr2.push(num);
                }
                Ordering::Equal => {
                    arr1.push(num);
                }
            },
        }
    }
    let ans = vec![arr1, arr2].concat();
    ans
}

fn greater_count(arr: &Vec<i32>, num: i32) -> i32 {
    let mut ans = 0;
    for n in arr {
        if *n > num {
            ans += 1;
        }
    }
    ans
}

#[test]
fn test_1() {
    assert_eq!(result_array(vec![2, 1, 3, 3]), vec![2, 3, 1, 3])
}

#[test]
fn test_2() {
    assert_eq!(result_array(vec![5, 14, 3, 1, 2]), vec![5, 3, 1, 2, 14])
}
