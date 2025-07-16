/// 3201. 找出有效子序列的最大长度I;

pub fn maximum_length_3201(nums: Vec<i32>) -> i32 {

    if nums.len() < 3 {
        return nums.len() as i32;
    }

    let mut total = Vec::new();

    let one = nums.get(0);
    if let Some(one) = one {
        total.push(one);
    }

    let mut n = 1;
    while n < nums.len() - 1 {
        if let Some(num) = nums.get(n) {
            if let Some(last) = total.last() {
                if let Some(next) = nums.get(n+1) {
                    if (*last + *num) % 2 == (*num + *next) % 2 {
                        total.push(num);
                        total.push(next);
                    } else {
                        total.pop();
                        total.pop();
                    }
                }
            }
        }
        n += 1;
    }

    total.len() as i32
}


#[test]
fn test_1() {
    assert_eq!(maximum_length_3201(vec![1,2,3,4]), 4);
}
#[test]
fn test_2() {
    assert_eq!(maximum_length_3201(vec![1,2,1,1,2,1,2]), 6);
}
#[test]
fn test_3() {
    assert_eq!(maximum_length_3201(vec![1, 3, 5]), 3);
}
#[test]
fn test_4() {
    assert_eq!(maximum_length_3201(vec![4,2,6]), 3);
}

#[test]
fn test_5() {
    assert_eq!(maximum_length_3201(vec![1,7,8,7,5]), 4);
}
