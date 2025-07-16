pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    if tomato_slices < cheese_slices * 2 {
        return vec![];
    }

    let mut tomato_num = -1;
    for i in 0..=cheese_slices {
        if i * 4 + (cheese_slices - i) * 2 == tomato_slices {
            tomato_num = i;
            break;
        }
    }

    if tomato_num == -1 {
        return vec![];
    }

    vec![tomato_num, cheese_slices - tomato_num]
}

#[test]
fn test_two() {
    assert_eq!(num_of_burgers(16, 7), vec![1, 6]);
}

#[test]
fn test_one() {
    assert_eq!(num_of_burgers(17, 4), vec![]);
}
