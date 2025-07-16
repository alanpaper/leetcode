use std::cmp::Ordering;
use std::collections::HashMap;
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

#[derive(Debug)]
struct BinaryIndexedTree {
    tree: Vec<i32>,
    size: usize,
}

impl BinaryIndexedTree {
    pub fn new(size: usize) -> Self {
        BinaryIndexedTree {
            tree: vec![0; size + 1],
            size,
        }
    }

    fn add(&mut self, mut v: i32) {
        while v < self.size as i32 {
            self.tree[v as usize] += 1;
            v += v & -v;
        }
    }

    pub fn query(&self, index: usize) -> i32 {
        let mut idx = index;
        let mut sum = 0;
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & !(idx - 1);
        }
        sum
    }
}

fn result_array_v2(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let mut index = HashMap::new();
    for i in 0..n {
        index.insert(sorted_nums[i], (i + 1) as i32);
    }

    let mut arr1 = vec![nums[0]];
    let mut arr2 = vec![nums[1]];
    let mut tree1 = BinaryIndexedTree::new(n);
    let mut tree2 = BinaryIndexedTree::new(n);
    tree1.add(index[&nums[0]]);
    tree2.add(index[&nums[1]]);

    for i in 2..n {
        let count1 = arr1.len() as i32 - tree1.query(index[&nums[i]] as usize);
        let count2 = arr2.len() as i32 - tree2.query(index[&nums[i]] as usize);
        if count1 > count2 || (count1 == count2 && arr1.len() <= arr2.len()) {
            arr1.push(nums[i]);
            tree1.add(index[&nums[i]]);
        } else {
            arr2.push(nums[i]);
            tree2.add(index[&nums[i]]);
        }
    }

    println!("{:#?}", &tree1);
    println!("{:#?}", &tree2);

    arr1.extend(arr2);
    arr1
}

#[test]
fn test_1() {
    assert_eq!(result_array_v2(vec![2, 1, 3, 3]), vec![2, 3, 1, 4])
}

#[test]
fn test_2() {
    assert_eq!(result_array_v2(vec![5, 14, 3, 1, 2]), vec![5, 3, 1, 2, 14])
}
