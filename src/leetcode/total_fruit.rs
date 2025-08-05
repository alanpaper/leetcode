use std::collections::{HashMap};


/// 904. 水果成篮

pub fn total_fruit(fruits: Vec<i32>) -> i32 {

    let mut total = 0;
    let mut fruits_set = HashMap::new();

    if fruits.len() < 2 {
        return fruits.len() as i32;
    }

    let mut left = 0;
    for right in 0..fruits.len() {
        fruits_set.entry(fruits[right]).and_modify(|f| {*f+=1;}).or_insert(1);
        while fruits_set.len() > 2 {
            fruits_set.entry(fruits[left]).and_modify(|f| {*f-=1;});
            if let Some(l) = fruits_set.get(&fruits[left]) {
                if *l == 0 {
                    fruits_set.remove(&fruits[left]);
                }
            }
            left += 1;
        }
        total = total.max(right - left + 1);
    }

  total as i32 
}


#[test]
fn test_1() {
    let fruits = vec![1,2,1];
    assert_eq!(total_fruit(fruits), 3);
}

#[test]
fn test_2() {
    let fruits = vec![3,3,3,1,2,1,1,2,3,3,4];
    assert_eq!(total_fruit(fruits), 5);
}

#[test]
fn test_3() {
    let fruits = vec![1,2,3,2,2];
    assert_eq!(total_fruit(fruits), 4);
}
