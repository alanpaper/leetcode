/// 326. Power of Three

pub fn is_power_of_three(n: i32) -> bool {
    let mut current = n;

    if n == 1 {
        return true;
    }

    while current > 3 {
        if current % 3 != 0 {
            return false;
        }
        current /= 3;
    }
    if current == 3 {
        return true;
    }
    false
}


#[test]
fn test_1() {
    assert_eq!(is_power_of_three(45), false);
}
#[test]
fn test_2() {
    assert_eq!(is_power_of_three(9), true);
}
#[test]
fn test_3() {
    assert_eq!(is_power_of_three(0), false);
}
#[test]
fn test_4() {
    assert_eq!(is_power_of_three(27), true);
}

#[test]
fn test_5() {
    assert_eq!(is_power_of_three(19684), false);
}

