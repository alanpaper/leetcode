use std::collections::HashMap;

/// 2661.找出完整访问过一遍的行或列对应的arr下标i

pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let mut map_stack = HashMap::new();
    for x in mat.iter().enumerate() {
        map_stack.insert(format!("x{}", x.0), x.1.clone());
    }
    for i in 0..mat[0].len() {
        let mut stack = vec![];
        for x in mat.iter().enumerate() {
            let num = x.1.clone();
            stack.push(num[i]);
        }
        map_stack.insert(format!("y{}", i), stack);
    }

    let mut i = 0;
    for num in arr.iter().enumerate() {
        i = num.0;
        let mut ls_full = false;
        for stack in map_stack.clone() {
            if stack.1.contains(num.1) {
                match map_stack.get(&stack.0).as_mut() {
                    Some(&mut x) => {
                        if x.len() > 0 {
                            let mut s = vec![];
                            for &i in x {
                                if i == *num.1 {
                                    continue;
                                } else {
                                    s.push(i);
                                }
                            }
                            if s.len() == 0 {
                                ls_full = true;
                                break;
                            }
                            map_stack.insert(stack.0, s);
                        } else {
                            ls_full = true;
                            break;
                        }
                    }
                    None => {
                        ls_full = true;
                        break;
                    }
                }
            }
        }
        if ls_full {
            break;
        }
    }

    println!("{:?}", map_stack);

    i as i32
}

#[test]
fn _test() {
    let arr = vec![1, 3, 4, 2];
    let mat = vec![vec![1, 4], vec![2, 3]];
    assert_eq!(first_complete_index(arr, mat), 2);
}

#[test]
fn _test_one() {
    let arr = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
    let mat = vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]];
    assert_eq!(first_complete_index(arr, mat), 3);
}
